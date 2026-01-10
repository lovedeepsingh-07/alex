pub mod cli;
pub mod constants;
pub mod daemon;
pub mod error;
pub mod request;
pub mod response;

use clap::Parser;

#[tokio::main]
async fn main() {
    let logger_env = env_logger::Env::default().filter_or("RUST_LOG", "alex=debug");
    env_logger::init_from_env(logger_env);

    let cli_args = cli::CliArgs::parse();
    if cli_args.sub_command == cli::SubCommand::Daemon {
        let daemon = daemon::Daemon::new();
        match daemon.run().await {
            Ok(_) => {}
            Err(e) => {
                log::error!("{}", e.to_string());
            }
        };
        return;
    }

    let mut request = request::Request::new();
    match request.generate(cli_args.sub_command) {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(1);
        }
    };
    match request.send().await {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(1);
        }
    };
}
