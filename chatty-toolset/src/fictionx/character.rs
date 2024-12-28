use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::json_schema;

#[derive(Deserialize)]
pub struct CharacterArgs {
    pub character_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct CharacterOutput {
    pub reply: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Character error")]
pub struct CharacterError;

#[derive(Deserialize, Serialize)]
pub struct Character;
impl Tool for Character {
    const NAME: &'static str = "chat_with_character";

    type Error = CharacterError;
    type Args = CharacterArgs;
    type Output = CharacterOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "chat_with_character".to_string(),
            description: "Request an chat with character".to_string(),
            parameters: json_schema!(
                character_name: string,
            ),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let character_name = args.character_name;
        println!("character_name: {}", character_name);

        Ok(CharacterOutput {
            reply: "abc".into(),
        })
    }
}
