mod components;

use leptos::*;
use leptos_meta::*;

use crate::api::converse;
use crate::model::conversation::{Conversation, Message};
use components::history::MessageHistory;
use components::input::MessageInputField;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };

        set_conversation.update(move |c| c.messages.push(user_message));
        async move { converse(conversation()).await }
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/iron_llama.css"/>

        // sets the document title
        <Title text="Iron LLaMa"/>

        <MessageHistory conversation/>
        <MessageInputField send/>
    }
}
