use std::{env, fs};

use crate::RunAgent;
use async_trait::async_trait;
use chatty_gemini::{agent_cli::CLIAgent, cli_chatbot, cli_chatbot_prompt};
use chatty_toolset::fictionx::{character::Character, recommendation::Recommendation};
use rig::providers::gemini::completion::GEMINI_1_5_FLASH;

#[derive(Clone)]
pub struct FictionXCLI {
    agent: CLIAgent,
}

impl FictionXCLI {
    pub fn new() -> Self {
        Self {
            agent: CLIAgent::new(),
        }
    }
}

#[async_trait]
impl RunAgent for FictionXCLI {
    async fn run(&self) {
        // Get the current directory and construct paths to JSON files
        let current_dir = env::current_dir().unwrap();
        let plan_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_plan.json");
        let premise_path =
            current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_premise.json");
        let story_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_story.txt");

        let plan_data = fs::read_to_string(&plan_path).expect("Unable to read JSON file");
        let premise_data = fs::read_to_string(&premise_path).expect("Unable to read JSON file");
        let story_data = fs::read_to_string(&story_path).expect("Unable to read JSON file");

        let agent = self.agent.client
        .agent(GEMINI_1_5_FLASH)
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform operations.
            Follow these instructions closely. 
            1. Consider the user's request carefully and identify the core elements of the request.
            2. Select which tool among those made available to you is appropriate given the context. 
            3. This is very important: never perform the operation yourself and never give me the direct result. 
            4. This is very important: One time reply or call a tool once, only once!
            5. If you don't know which tool to run, ASK USER TO INPUT RIGHT COMMEND!
            "
        )
        .max_tokens(1024)
        // .context(&plan_data)
        // .context(&premise_data)
        // .context(&story_data)
        .tool(Recommendation)
        .tool(Character)
        .build();

        let _ = cli_chatbot_prompt(agent).await;
        // let _ = cli_chatbot(agent, "").await;
    }
}
