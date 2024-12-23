use rig::{completion::Prompt, providers::gemini};

pub async fn reply(prompt: &str) -> String {
    let gemini_client = gemini::Client::from_env();

    let agent = gemini_client
        .agent(gemini::completion::GEMINI_1_5_FLASH)
        .preamble("You are comedian AI with a mission to make people laugh.")
        .temperature(0.0)
        .build();

    // Prompt the model and print its response
    let response = agent
        .prompt(prompt)
        .await
        .expect("Failed to prompt gemini-1.5-flash");

    response
}
