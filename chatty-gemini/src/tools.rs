use rig::{
    completion::{Prompt, ToolDefinition},
    providers,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
pub struct OperationArgs {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Math error")]
pub struct MathError;

#[derive(Deserialize, Serialize)]
pub struct Adder;
impl Tool for Adder {
    const NAME: &'static str = "add";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "add".to_string(),
            description: "Add x and y together".to_string(),
            parameters: json!({
                    // "type": "OBJECT",

                   "parameters": {
              "type": "OBJECT",
              "properties": {
                "x": {
                  "type": "NUMBER"
                },
                "y": {
                  "type": "NUMBER"
                }
              },
              "required": [
                "x",
                "y"
              ]
            }
                }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!(">>> call: {} // {}", args.x, args.y);

        let result = args.x + args.y;
        Ok(result)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Subtract;
impl Tool for Subtract {
    const NAME: &'static str = "subtract";

    type Error = MathError;
    type Args = OperationArgs;
    type Output = i32;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        serde_json::from_value(json!({
            "name": "subtract",
            "description": "Subtract y from x (i.e.: x - y)",
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
        }))
        .expect("Tool Definition")
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = args.x - args.y;
        Ok(result)
    }
}