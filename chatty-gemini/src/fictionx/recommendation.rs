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

#[derive(Deserialize, Serialize)]
pub struct RecommendationResponse {
    #[serde(rename = "job_id")]
    pub job_id: String,

    #[serde(rename = "user_id")]
    pub user_id: String,

    #[serde(rename = "user_nickname")]
    pub user_nickname: String,

    #[serde(rename = "model")]
    pub model: String,

    #[serde(rename = "cover_img")]
    pub cover_img: String,

    #[serde(rename = "cover_img_status")]
    pub cover_img_status: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "premise")]
    pub premise: String,

    #[serde(rename = "setting")]
    pub setting: String,

    #[serde(rename = "genres")]
    pub genres: String,

    #[serde(rename = "latest_chapter_num")]
    pub latest_chapter_num: i32,

    #[serde(rename = "total_likes")]
    pub total_likes: i64,

    #[serde(rename = "self_collect")]
    pub self_collect: bool,

    #[serde(rename = "self_like")]
    pub self_like: bool,

    #[serde(rename = "total_collects")]
    pub total_collects: i64,

    #[serde(rename = "collection_id")]
    pub collection_id: String,

    #[serde(rename = "created_time")]
    pub created_time: DateTime<Utc>,

    #[serde(rename = "updated_time")]
    pub updated_time: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
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

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let mut headers = HeaderMap::new();
        headers.insert(CONNECTION, HeaderValue::from_static("CLOSE"));

        let response = reqwest::Client::new()
            .post("https://fictionx.ai/api/story/recommendation")
            .headers(headers)
            .send()
            .await
            .unwrap();

        let json_response: serde_json::Value = response.json().await.unwrap();
        // println!("Response: {:#?}", json_response);

        let stories = &json_response["data"]["stories"];
        let mut titles = Vec::new();

        for (index, story) in stories.as_array().unwrap().iter().enumerate() {
            if let Some(title) = story.get("title") {
                let title = format!("> {}. {}", index + 1, title.as_str().unwrap().to_string());
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
}
