pub(crate) mod cli;
pub(crate) mod command;
pub(crate) mod constants;
pub(crate) mod daemon;
pub(crate) mod error;
pub(crate) mod player;
pub(crate) mod protocol;

use clap::Parser;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let logger_env = env_logger::Env::default().filter_or("RUST_LOG", "alex=debug");
    env_logger::init_from_env(logger_env);

    let cli_args = cli::CliArgs::parse();
    if cli_args.sub_command == cli::SubCommand::Daemon {
        match daemon::run().await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to run daemon, {}", e.to_string());
            }
        }
        return;
    }

    match connect(cli_args.sub_command).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to make request, {}", e.to_string());
        }
    };
}

async fn connect(sub_command: cli::SubCommand) -> Result<(), error::Error> {
    let request = cli::generate_request(sub_command)?;
    let mut tcp_stream = tokio::net::TcpStream::connect(constants::SERVER_ADDRESS).await?;
    tcp_stream.write_all(&request.to_bytes()).await?;

    // NOTE: this is important to ensure anyone reading from this stream does not loop
    // forever, it ensure EOF is reached on the "reader" side, by shutting down the "writer"
    tcp_stream.shutdown().await?;

    let response = protocol::response::Response::from_stream(&mut tcp_stream).await?;
    println!("{:#?}", response);
    if let Some(command_line) = response.data.get(1) {
        match command_line.as_str() {
            "STATUS" => {
                if let Some(status_line) = response.data.get(2) {
                    println!("{:#?}", serde_json::from_str::<player::PlayerState>(&status_line).unwrap())
                }
            },
            _ => {}
        }
    }

    Ok(())
}
