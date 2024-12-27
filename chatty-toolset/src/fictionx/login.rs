use chatty_config::CHATTY_CONFIG;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct LoginArgs {
    pub public_address: String,
    pub signature: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginOutput {
    pub reply: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Login error")]
pub struct LoginError;

#[derive(Deserialize, Serialize)]
pub struct Login;
impl Tool for Login {
    const NAME: &'static str = "login";

    type Error = LoginError;
    type Args = LoginArgs;
    type Output = LoginOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "login".to_string(),
            description: "Help me login FictionX using Solana account".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "public_address": {
                        "type": "string",
                    },
                    "signature": {
                        "type": "string",
                    }
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("Login info: {} / {}", args.public_address, args.signature);

        let url = CHATTY_CONFIG.fictionx_service.base_endpoint.clone()
            + &CHATTY_CONFIG.fictionx_service.path_login;
        let response = reqwest::Client::new().post(url).json(&args).send().await;

        if let Ok(res) = response {
            let json_response: serde_json::Value = res.json().await.unwrap();
            println!("json_response: {:?}", json_response);
            let jwt_token = &json_response["data"]["jwt_token"];

            return Ok(LoginOutput {
                reply: jwt_token.to_string(),
            });
        }

        Err(LoginError)
    }
}
