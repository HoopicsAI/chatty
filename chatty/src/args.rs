use clap::Parser;
use colored::Colorize;

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
        // 返回错误提示
        eprintln!(
            "{}",
            "*** Please specify either --cli or --api to choose the running mode ***"
                .red()
                .bold()
        );
        std::process::exit(1);
    }
}
