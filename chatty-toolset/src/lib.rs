pub mod fictionx;
pub mod registry;

// Best Practices
// Unique Names: Ensure each tool has a unique name within your application
// Clear Descriptions: Provide clear, detailed descriptions in tool definitions
// Type Safety: Use strong typing for tool arguments and outputs
// Error Handling: Implement proper error types and handling
// RAG Consideration: Consider implementing ToolEmbedding if your tool might benefit from semantic retrieval

#[macro_export]
macro_rules! json_schema {
    ($($name:ident: $type:tt),* $(,)?) => {{
        let mut properties = serde_json::Map::new();
        $(
            let property = match stringify!($type) {
                "String" => json!({
                    "type": "string"
                }),
                "i32" | "i64" | "u64" | "u32" => json!({
                    "type": "number"
                }),
                "bool" => json!({
                    "type": "boolean"
                }),
                _ => panic!("Unsupported type"),
            };
            properties.insert(stringify!($name).to_string(), property);
        )*
        json!({
            "type": "object",
            "properties": properties,
        })
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_schema_works() {
        let schema = json_schema!(
            character_name: String,
            age: i32,
            is_student: bool,
        );

        println!("{}", schema.to_string());
    }
}
