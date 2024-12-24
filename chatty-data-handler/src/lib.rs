use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Entity {
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutlineChild {
    text: String,
    scene: String,
    entities: Vec<String>,
    children: Vec<OutlineChild>,
    id: Option<String>, // ID 是可选的
}

#[derive(Debug, Serialize, Deserialize)]
struct Outline {
    text: String,
    scene: String,
    entities: Vec<String>,
    children: Vec<OutlineChild>,
    id: Option<String>, // ID 是可选的
}

#[derive(Debug, Serialize, Deserialize)]
struct Premise {
    title: String,
    premise: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Story {
    premise: Premise,
    setting: String,
    entities: Vec<Entity>,
    outline: Outline,
}

use rig::embeddings::EmbeddingsBuilder;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::{completion::Prompt, providers::gemini};
use std::env;
use std::fs;
pub async fn store() {
    let mut vector_store = InMemoryVectorStore::default();

    let gemini_client = gemini::Client::from_env();

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
}

#[cfg(test)]
mod tests {

    #[test]
    fn handle_data_works() {
        use super::*;
        use std::fs::File;
        use std::path::PathBuf;

        // 读取 JSON 文件
        let mut path = PathBuf::from(std::env::current_dir().unwrap());
        println!("path: {:?}", path);
        path.push("f36a5a1b-a698-41b6-947c-57cc5e5add31_plan.json");
        println!("path: {:?}", path);

        let file = File::open(path).expect("Unable to open file");

        // 将文件内容解析为 Story 结构体
        let story: Story = serde_json::from_reader(file).unwrap();

        // 打印解析结果
        println!("Title: {}", story.premise.title);
        println!("Premise: {}", story.premise.premise);
        println!("Setting: {}", story.setting);

        for entity in story.entities {
            println!("Entity Name: {}", entity.name);
            println!("Entity Description: {}", entity.description);
            println!("---------------------------");
        }

        println!("Outline Text: {}", story.outline.text);

        for child in &story.outline.children {
            println!("Child Text: {}", child.text);
            println!("Scene: {}", child.scene);
            println!("Entities in Child: {:?}", child.entities);
            println!("---------------------------");

            for grandchild in &child.children {
                println!("  Grandchild Text: {}", grandchild.text);
                println!("  Grandchild Scene: {}", grandchild.scene);
                println!("  Entities in Grandchild: {:?}", grandchild.entities);
                println!("  ---------------------------");
            }
        }
    }
}
