pub mod ascii;

use chatty_gemini::recommend;
use chatty_gemini::{add, reply_cli};
use chatty_prompt::create_prompt;
use colored::Colorize;

pub async fn main() -> std::io::Result<()> {
    println!("{}", "Chatty running in CLI mode".yellow());

    // let prompt = create_prompt!(name, description, "");
    // reply_cli(&prompt).await;

    // add().await;

    recommend().await;

    Ok(())
}
