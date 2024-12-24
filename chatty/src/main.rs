use clap::{Arg, Command};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 构建命令行参数解析器
    let matches = Command::new("chatty")
        .version("1.0")
        .author("zTgx")
        .about("FictionX ChatBot: chat with role")
        .subcommand(
            Command::new("cli")
                .about("Run the CLI mode of the application")
                .arg(
                    Arg::new("name")
                        .help("The name of the role")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("description")
                        .help("The description of the role")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    // 判断是否执行CLI模式
    if let Some(cli_matches) = matches.subcommand_matches("cli") {
        // 获取并使用命令行参数
        let name = cli_matches
            .get_one::<String>("name")
            .expect("Failed to get name argument");
        let description = cli_matches
            .get_one::<String>("description")
            .expect("Failed to get description argument");

        println!("Running in CLI mode");
        println!("Character Name: {}", name);
        println!("Character Description: {}", description);

        // 根据实际需求完善CLI逻辑，比如更多参数处理、具体功能执行等
        chatty_cli::cli(&name, &description).await;
    } else {
        // 启动服务逻辑，调用之前的chatty_service::main().await
        let _ = chatty_service::main().await;
    }
    Ok(())
}
