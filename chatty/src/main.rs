#[tokio::main]
async fn main() -> std::io::Result<()> {
    chatty_service::main().await
}