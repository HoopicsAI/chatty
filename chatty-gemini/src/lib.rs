use rig::completion::Prompt;
use rig::embeddings::EmbeddingsBuilder;
use rig::providers::gemini::{completion::GEMINI_1_5_FLASH, embedding::EMBEDDING_004, Client};
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use std::env;
use std::fs;

pub async fn reply(prompt: &str) -> String {
    let gemini_client = Client::from_env();

    let mut vector_store = InMemoryVectorStore::default();

    // Create an embedding model using OpenAI's text-embedding-ada-002
    let embedding_model = gemini_client.embedding_model(EMBEDDING_004);

    // Get the current directory and construct paths to JSON files
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let path = current_dir.join("f36a5a1b-a698-41b6-947c-57cc5e5add31_plan.json");
    println!("Loading JSON from path: {:?}", path);

    let json_data = fs::read_to_string(&path).expect("Unable to read JSON file");
    println!("Loaded JSON data: {}", json_data);

    // Create embeddings for the JSON contents
    let embeddings_result = EmbeddingsBuilder::new(embedding_model.clone())
        .document(json_data)
        .unwrap()
        .build()
        .await;

    match embeddings_result {
        Ok(embeddings) => {
            // Add the embeddings to our vector store
            vector_store.add_documents(embeddings);
        }
        Err(e) => {
            eprintln!("Failed to create embeddings: {:?}", e);
            return "Failed to create embeddings.".to_string();
        }
    }

    let c = r#"You are a helpful assistant. Check your knowledge base before answering any questions.
    Only respond to questions using information from tool calls.
    if no relevant information is found in the tool calls, respond, "Sorry, I don't know.""#;

    let agent = gemini_client
        .agent(GEMINI_1_5_FLASH)
        .preamble(c)
        .dynamic_context(2, vector_store.index(embedding_model))
        .temperature(0.0)
        .build();

    // Prompt the model and print its response
    let response_result = agent.prompt(prompt).await;

    match response_result {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to prompt gemini-1.5-flash: {:?}", e);
            "Failed to get a response.".to_string()
        }
    }
}
