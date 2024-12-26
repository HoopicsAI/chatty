pub mod agents;
pub mod ascii;

use agents::fictionx::FictionXCLI;
use async_trait::async_trait;
use colored::Colorize;
use std::sync::Arc;

pub async fn main() -> std::io::Result<()> {
    println!("{}", "Chatty running in CLI mode".yellow());

    let fictionx_cli = Arc::new(FictionXCLI::new());
    let _ = fictionx_cli.run().await;

    Ok(())
}

#[async_trait]
pub trait RunAgent {
    async fn run(&self);
}
