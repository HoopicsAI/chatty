use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ChatMessage {
    pub character_id: String,
    pub message: String,
}