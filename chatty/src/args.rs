use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    cli: bool,

    #[clap(long)]
    api: bool,
}

pub enum RunningMode {
    API,
    CLI,
}

pub fn mode() -> RunningMode {
    let args = Args::parse();
    if args.cli {
        RunningMode::CLI
    } else if args.api {
        RunningMode::API
    } else {
        // 这里可以根据实际需求修改默认返回逻辑，比如返回错误提示或者按照默认设定的模式返回
        eprintln!("Please specify either --cli or --api to choose the running mode.");
        std::process::exit(1);
    }
}
