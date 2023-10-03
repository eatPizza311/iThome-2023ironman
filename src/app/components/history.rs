use leptos::{html::Div, *};

use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "message user bg-emerald-300 border-emerald-300 self-end";
const MODEL_MESSAGE_CLASS: &str = "message assistant bg-blue-100 border-blue-100 self-start";
const DOT_3_CLASS: &str =
    "rounded-lg py-2.5 px-6 mb-4 message assistant bg-blue-100 border-blue-100 self-start";

#[component]
pub fn MessageHistory(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>();

    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div class="h-screen pb-24 w-full flex flex-col overflow-y-auto p-5 border-gray-300 bg-gray-100" node_ref=chat_div_ref>
            {move || conversation.get().messages.iter().map(move |message| {
                let class_str = if message.user { USER_MESSAGE_CLASS } else { MODEL_MESSAGE_CLASS };
                // ... wave while waiting for response
                if message.text == String::from("...") {
                    view! {
                        <div class=DOT_3_CLASS>
                            <div class="type-indicator">
                                <span>.</span><span>.</span><span>.</span>
                            </div>
                        </div>
                    }
                } else {
                    view! {
                        <div class=class_str>
                            {message.text.clone()}
                        </div>
                    }
                }
            }).collect::<Vec<_>>()
            }
        </div>
    }
}
