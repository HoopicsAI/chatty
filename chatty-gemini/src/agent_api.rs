use rig::completion::{Chat, Message};
use rig::providers::gemini::{completion::GEMINI_1_5_FLASH, Client};
use std::{fs, env};

pub struct APIAgent {
    pub client: Client,
    pub model: String,
}

impl APIAgent {
    pub fn new() -> Self {
        Self {
            client: Client::from_env(),
            model: GEMINI_1_5_FLASH.into(),
        }
    }
}

impl APIAgent {
    pub async fn chat(&self, prompt: &str, message: &str) -> String {
        let current_dir = env::current_dir().unwrap();
        let plan_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_plan.json");
        let premise_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_premise.json");
        let story_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_story.txt");

        let plan_data = fs::read_to_string(&plan_path).expect("Unable to read JSON file");
        let premise_data = fs::read_to_string(&premise_path).expect("Unable to read JSON file");
        let story_data = fs::read_to_string(&story_path).expect("Unable to read JSON file");

        let agent = self.client
            .agent(&self.model)
            .preamble("Be creative and concise. Answer directly and clearly.")
            .context(&plan_data)
            .context(&premise_data)
            .context(&story_data)
            .temperature(0.0)
            .build();

        let message = Message {
            role: "user".into(),
            content: message.into(),
        };
        let response_result = agent.chat(prompt, vec![message]).await;
        match response_result {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to prompt gemini-1.5-flash: {:?}", e);
                "Failed to get a response.".to_string()
            }
        }
    }
}