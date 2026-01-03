mod cli;
mod constants;
mod daemon;
mod error;
mod status;

use clap::Parser;

#[tokio::main]
async fn main() {
    let cli_args = cli::CliArgs::parse();
    match cli_args.sub_command {
        cli::SubCommand::Daemon => {
            let daemon = daemon::Daemon::new();
            match daemon.run().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e.to_string());
                }
            };
        }
        cli::SubCommand::Status => {
            match status::run().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e.to_string());
                }
            };
        }
        cli::SubCommand::Search => {
            println!("search for a song");
        }
    }
}
