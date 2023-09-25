use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub message: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            message: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    // distinguish the message from the LLM and the user
    pub user: bool,
    // the message
    pub text: String,
}
