use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChatMessage {
    // 可选
    // 默认是 gemini
    pub model: Option<String>,

    pub character_id: String,
    pub character_name: String,
    pub description: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChatMessageReply {
    pub response: String,
}

impl ChatMessageReply {
    pub fn new(response: String) -> Self {
        Self { response }
    }
}
