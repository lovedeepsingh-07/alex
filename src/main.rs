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
    handle_response(response).await?;

    Ok(())
}

async fn handle_response(response: protocol::response::Response) -> Result<(), error::Error> {
    let result_line = response
        .data
        .get(0)
        .ok_or_else(|| error::Error::ProtocolError("Missing the result line".to_string()))?;
    match result_line.as_str() {
        "OK" => {}
        "ERROR" => {
            if let Some(error_value) = response.data.get(1) {
                return Err(error::Error::ProtocolError(format!("{}", error_value)));
            }
            return Err(error::Error::ProtocolError("ERROR".to_string()));
        }
        _ => {
            return Err(error::Error::ProtocolError(
                "Invalid result line".to_string(),
            ));
        }
    }

    let command_line = response
        .data
        .get(1)
        .ok_or_else(|| error::Error::ProtocolError("Missing the command line".to_string()))?;
    match command_line.as_str() {
        "STATUS" => {
            let status_sub_command_line = response.data.get(2).ok_or_else(|| {
                error::Error::ProtocolError("Missing the status sub command line".to_string())
            })?;
            match status_sub_command_line.as_str() {
                "ALL" => {
                    let status_json = response.data.get(3).ok_or_else(|| {
                        error::Error::ProtocolError("Missing the status json".to_string())
                    })?;
                    println!("{}", status_json);
                },
                "CURRENT_AUDIO" => {
                    if let Some(current_audio) = response.data.get(3){
                        println!("{}", current_audio);
                    } else {
                        println!("NO AUDIO");
                    }
                },
                "IS_PAUSED" => {
                    let is_paused = response.data.get(3).ok_or_else(|| {
                        error::Error::ProtocolError("Missing is_paused".to_string())
                    })?;
                    println!("{}", is_paused);
                },
                "IS_QUEUE_EMPTY" => {
                    let is_queue_empty = response.data.get(3).ok_or_else(|| {
                        error::Error::ProtocolError("Missing the is_queue_empty".to_string())
                    })?;
                    println!("{}", is_queue_empty);
                },
                _ => {
                    return Err(error::Error::ProtocolError(
                        "Invalid status sub command line".to_string(),
                    ));
                }
            }
        }
        "RELOAD" => {
            println!("> Player reloaded");
        }
        "SEARCH" => {
            println!("> Searching");
        }
        "PLAYER" => {
            println!("> Player");
        }
        _ => {
            return Err(error::Error::ProtocolError(
                "Invalid command line".to_string(),
            ));
        }
    }

    Ok(())
}
