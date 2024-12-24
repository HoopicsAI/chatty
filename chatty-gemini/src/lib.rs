use rig::embeddings::EmbeddingsBuilder;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::{completion::Prompt, providers::gemini};
use std::env;
use std::fs;

pub async fn reply(prompt: &str) -> String {
    let gemini_client = gemini::Client::from_env();

    let mut vector_store = InMemoryVectorStore::default();

    // Create an embedding model using OpenAI's text-embedding-ada-002
    let embedding_model = gemini_client.embedding_model(gemini::embedding::EMBEDDING_001);

    // Get the current directory and construct paths to PDF files
    let current_dir = env::current_dir().unwrap();

    println!("path: {:?}", current_dir);
    let path = current_dir.join("f36a5a1b-a698-41b6-947c-57cc5e5add31_plan.json");
    println!("path: {:?}", path);

    let json_data = fs::read_to_string(path).unwrap();

    // Create embeddings for the PDF contents
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .document(json_data)
        .unwrap()
        .build()
        .await
        .unwrap();

    // Add the embeddings to our vector store
    vector_store.add_documents(embeddings);

    let agent = gemini_client
        .agent(gemini::completion::GEMINI_1_5_FLASH)
        .preamble("You are a helpful assistant that answers questions based on the given context.")
        .dynamic_context(2, vector_store.index(embedding_model))
        .temperature(0.0)
        .build();

    // Prompt the model and print its response
    let response = agent
        .prompt(prompt)
        .await
        .expect("Failed to prompt gemini-1.5-flash");

    response
}
