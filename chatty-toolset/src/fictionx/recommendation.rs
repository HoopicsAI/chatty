use chatty_config::CHATTY_CONFIG;
use reqwest::header::{HeaderMap, HeaderValue, CONNECTION};
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use chatty_data_handler::ChattyDataTag;

#[derive(Deserialize)]
pub struct RecommendationArgs {}

#[derive(Deserialize, Serialize)]
pub struct RecommendationOutput {
    pub id: String,
    pub recommend_novels: Vec<String>,
}

impl ChattyDataTag for RecommendationOutput {
    fn tag(&self) -> uuid::Uuid {
        Uuid::parse_str(&self.id).unwrap()
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Recommendation error")]
pub struct RecommendationError;

#[derive(Deserialize, Serialize)]
pub struct Recommendation;
impl Tool for Recommendation {
    const NAME: &'static str = "story_recommendation";

    type Error = RecommendationError;
    type Args = RecommendationArgs;
    type Output = RecommendationOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "story_recommendation".to_string(),
            description: "Fetch the recommended novels on FictionX".to_string(),
            parameters: json!({
                "parameters": {
                   "type": "object",
                    "properties": {
                        "dummy": {
                            "type": "string",
                        }
                    }
                }
            }),
        }
    }

    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONNECTION, HeaderValue::from_static("CLOSE"));

        let url = CHATTY_CONFIG.fictionx_service.base_endpoint.clone()
            + &CHATTY_CONFIG.fictionx_service.path_recommend;
        let response = reqwest::Client::new()
            .post(url)
            .headers(headers)
            .send()
            .await;
        match response {
            Ok(res) => {
                let json_response: serde_json::Value = res.json().await.unwrap();
                let stories = &json_response["data"]["stories"];
                let mut titles = Vec::new();

                for (index, story) in stories.as_array().unwrap().iter().enumerate() {
                    if let Some(title) = story.get("title") {
                        let title =
                            format!("> {}. {}", index + 1, title.as_str().unwrap().to_string());
                        titles.push(title);
                    }
                }

                Ok(RecommendationOutput {
                    id: Uuid::new_v4().to_string(),
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
