use crate::RunAgent;
use async_trait::async_trait;
use chatty_gemini::{agent_cli::CLIAgent, cli_chatbot_prompt};
use chatty_toolset::fictionx::recommendation::Recommendation;
use colored::Colorize;
use rig::providers::gemini::completion::GEMINI_1_5_FLASH;
use rig::providers::gemini::Client;

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
        println!("{}", "FictionX Agent is running...".red());

        let agent = self.agent.client
        .agent(GEMINI_1_5_FLASH)
        .preamble(
            "You are an assistant here to help the user select which tool is most appropriate to perform arithmetic operations.
            Follow these instructions closely. 
            1. Consider the user's request carefully and identify the core elements of the request.
            2. Select which tool among those made available to you is appropriate given the context. 
            3. This is very important: never perform the operation yourself and never give me the direct result. 
            "
        )
        .max_tokens(1024)
        .tool(Recommendation)
        .build();

        let _ = cli_chatbot_prompt(agent).await;
    }
}

pub async fn xxx() {
    println!("{}", "FictionX Agent is running...".red());

    let agent = Client::from_env()
    .agent(GEMINI_1_5_FLASH)
    .preamble(
        "You are an assistant here to help the user select which tool is most appropriate to perform arithmetic operations.
        Follow these instructions closely. 
        1. Consider the user's request carefully and identify the core elements of the request.
        2. Select which tool among those made available to you is appropriate given the context. 
        3. This is very important: never perform the operation yourself and never give me the direct result. 
        "
    )
    .max_tokens(1024)
    .tool(Recommendation)
    .build();

    let _ = cli_chatbot_prompt(agent).await;
}
