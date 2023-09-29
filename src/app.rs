use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::model::conversation::{Conversation, Message};

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
        async move { todo!() }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/iron_llama.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // <MessageHistory conversation/>
        // <MessageInputField send/>
    }
}
