use chatty_gemini::{add, reply_cli};
use chatty_prompt::create_prompt;

pub async fn cli(name: &str, description: &str) {
    // let prompt = create_prompt!(name, description, "");
    // reply_cli(&prompt).await;

    add().await;
}
