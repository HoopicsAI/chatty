use chatty_data_handler::ChattyDataHandler;
use rig::completion::{Chat, Message, Prompt, PromptError};
use std::io::{self, Write};

pub mod agent_api;
pub mod agent_cli;

// pub async fn reply_cli(prompt: &str) {
//     let gemini_client = Client::from_env();

//     // Get the current directory and construct paths to JSON files
//     let current_dir = env::current_dir().unwrap();
//     let plan_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_plan.json");
//     let premise_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_premise.json");
//     let story_path = current_dir.join("sample/dbafa0ef-717d-4f65-978c-84e19580618f_story.txt");

//     let plan_data = fs::read_to_string(&plan_path).expect("Unable to read JSON file");
//     let premise_data = fs::read_to_string(&premise_path).expect("Unable to read JSON file");
//     let story_data = fs::read_to_string(&story_path).expect("Unable to read JSON file");

//     let agent = gemini_client
//         .agent(GEMINI_1_5_FLASH)
//         .preamble(prompt)
//         .context(&plan_data)
//         .context(&premise_data)
//         .context(&story_data)
//         .temperature(0.0)
//         .build();

//     let _ = cli_chatbot(agent, prompt).await;
// }

pub async fn cli_chatbot(chatbot: impl Chat, prompt: &str) -> Result<(), PromptError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut chat_log = vec![];

    println!("Welcome to the FictionX chatbot! Type 'exit' to quit.");
    loop {
        print!("> ");
        // Flush stdout to ensure the prompt appears before input
        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                // Remove the newline character from the input
                let input = input.trim();
                // Check for a command to exit
                if input == "exit" {
                    break;
                }
                // tracing::info!("Prompt:\n{}\n", input);

                let prompt = prompt.to_string() + input;
                // println!("prompt: {}", prompt);
                let response = chatbot.chat(&prompt, chat_log.clone()).await?;
                chat_log.push(Message {
                    role: "user".into(),
                    content: input.into(),
                });
                chat_log.push(Message {
                    role: "assistant".into(),
                    content: response.clone(),
                });

                println!("========================== Response ============================");
                println!("{response}");
                println!("================================================================\n\n");

                // tracing::info!("Response:\n{}\n", response);
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }

    Ok(())
}

pub async fn cli_chatbot_prompt(chatbot: impl Prompt) -> Result<(), PromptError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let data_handler = ChattyDataHandler::new();
    println!("Welcome to the Chatty chatbot! Type 'exit' to quit.\n");
    loop {
        print!("Alice > ");

        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(input_len) => {
                // Remove the newline character from the input
                let input = input.trim();
                if input.is_empty() || input_len < 1 {
                    println!("");

                    continue;
                }

                // Check for a command to exit
                if input == "exit" {
                    break;
                }

                println!("========================== Response ============================");
                let response = chatbot.prompt(&input).await;
                match response {
                    Ok(data) => {
                        data_handler.handler(&data);
                        println!("Agent > {}", data);
                    },
                    Err(e) => println!("Error: {:?}", e),
                }
                println!("================================================================\n\n");
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }

    Ok(())
}
