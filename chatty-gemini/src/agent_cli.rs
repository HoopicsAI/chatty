use rig::providers::gemini::{completion::GEMINI_1_5_FLASH, Client};

#[derive(Clone)]
pub struct CLIAgent {
    pub client: Client,
    pub model: String,
}

impl CLIAgent {
    pub fn new() -> Self {
        Self {
            client: Client::from_env(),
            model: GEMINI_1_5_FLASH.into(),
        }
    }
}
