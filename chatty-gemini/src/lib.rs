use rig::completion::{Chat, Message, Prompt};
use rig::providers::gemini::{completion::GEMINI_1_5_FLASH, Client};
use std::env;
use std::fs;

pub async fn reply(prompt: &str) -> String {
    let gemini_client = Client::from_env();

    // Get the current directory and construct paths to JSON files
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let plan_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_plan.json");
    let premise_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_premise.json");
    let story_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_story.txt");

    let plan_data = fs::read_to_string(&plan_path).expect("Unable to read JSON file");
    let premise_data = fs::read_to_string(&premise_path).expect("Unable to read JSON file");
    let story_data = fs::read_to_string(&story_path).expect("Unable to read JSON file");

    let c = r#"You are one of the roles in this contexts, Engage in a conversation as if you were her."#;

    let agent = gemini_client
        .agent(GEMINI_1_5_FLASH)
        .preamble(c)
        .context(&plan_data)
        .context(&premise_data)
        .context(&story_data)
        .temperature(0.0)
        .build();

    // Prompt the model and print its response
    let message = Message {
        role: "user".into(),
        content: prompt.into(),
    };
    let response_result = agent.chat(&c, vec![message]).await;

    match response_result {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to prompt gemini-1.5-flash: {:?}", e);
            "Failed to get a response.".to_string()
        }
    }
}
