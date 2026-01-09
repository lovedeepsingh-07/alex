mod cli;
mod constants;
mod daemon;
mod error;

use tokio::io::AsyncWriteExt;
use clap::Parser;

async fn handle_player_subcommand(player_args: cli::PlayerArgs) -> Result<(), error::Error> {
    let mut stream = tokio::net::TcpStream::connect(constants::SERVER_ADDRESS).await?;
    let mut response = String::from("PLAYER:");

    match player_args.sub_command {
        cli::PlayerSubCommand::Play => {
            response.push_str("PLAY");
        },
        cli::PlayerSubCommand::Pause => {
            response.push_str("PAUSE");
        },
        cli::PlayerSubCommand::Resume => {
            response.push_str("RESUME");
        },
        cli::PlayerSubCommand::Clear => {
            response.push_str("CLEAR");
        },
    }

    Ok(stream.write_all(&response.as_bytes()).await?)
}

#[tokio::main]
async fn main() {
    let logger_env = env_logger::Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(logger_env);

    let cli_args = cli::CliArgs::parse();
    match cli_args.sub_command {
        cli::SubCommand::Daemon => {
            let daemon = daemon::Daemon::new();
            match daemon.run().await {
                Ok(_) => {}
                Err(e) => {
                    log::error!("{}", e.to_string());
                }
            };
        }
        cli::SubCommand::Player(player_args) => {
            match handle_player_subcommand(player_args).await {
                Ok(_) => {},
                Err(e) => {
                    log::error!("{}", e.to_string());
                }
            }
        },
    }
}
