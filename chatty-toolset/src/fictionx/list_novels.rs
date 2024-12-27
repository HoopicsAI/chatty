use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct ListNovelsArgs {
    pub count: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ListNovelsOutput {
    pub reply: String,
}

#[derive(Debug, thiserror::Error)]
#[error("ListNovels error")]
pub struct ListNovelsError;

#[derive(Deserialize, Serialize)]
pub struct ListNovels;
impl Tool for ListNovels {
    const NAME: &'static str = "list_novels";

    type Error = ListNovelsError;
    type Args = ListNovelsArgs;
    type Output = ListNovelsOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "list_novels".to_string(),
            description: "Show me {number} count of novels".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "count": {
                        "type": "number",
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let count = args.count;
        println!("ListNovels count: {}", count);

        Ok(ListNovelsOutput {
            reply: count.to_string(),
        })
    }
}
