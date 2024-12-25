mod args;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    chatty_cli::ascii::print();

    match args::mode() {
        args::RunningMode::API => {
            chatty_service::main().await?;
        }
        args::RunningMode::CLI => {
            chatty_cli::main().await?;
        }
    }

    Ok(())
}
