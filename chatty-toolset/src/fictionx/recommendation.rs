use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, CONNECTION};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct RecommendationArgs {}

#[derive(Deserialize, Serialize)]
pub struct RecommendationOutput {
    pub recommend_novels: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("Recommendation error")]
pub struct RecommendationError;

#[derive(Deserialize, Serialize)]
pub struct Recommendation;
impl Tool for Recommendation {
    const NAME: &'static str = "recommend";

    type Error = RecommendationError;
    type Args = RecommendationArgs;
    type Output = RecommendationOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "recommend".to_string(),
            description: "Get the recommended novels on FictionX".to_string(),
            parameters: json!({
                "parameters": {
                   "type": "object",
                "properties": {
                    "x": {
                        "type": "number",
                        "description": "The number to substract from"
                    },
                    "y": {
                        "type": "number",
                        "description": "The number to substract"
                    }
                }
                }
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONNECTION, HeaderValue::from_static("CLOSE"));

        let response = reqwest::Client::new()
            .post("https://fictionx.ai/api/story/recommendation")
            .headers(headers)
            .send()
            .await;
        match response {
            Ok(res) => {
                let json_response: serde_json::Value = res.json().await.unwrap();
                // println!("Response: {:#?}", json_response);

                let stories = &json_response["data"]["stories"];
                let mut titles = Vec::new();

                for (index, story) in stories.as_array().unwrap().iter().enumerate() {
                    if let Some(title) = story.get("title") {
                        let title =
                            format!("> {}. {}", index + 1, title.as_str().unwrap().to_string());
                        println!("{}", title);
                        titles.push(title);
                    }
                }

                // 打印提取的 titles
                // println!("titles: {:?}", titles);

                Ok(RecommendationOutput {
                    recommend_novels: titles,
                })
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                return Err(RecommendationError);
            }
        }
    }
}
