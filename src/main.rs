use alex::{cli, daemon, error, protocol};

use clap::Parser;
use colored::Colorize;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    env_logger::Builder::new()
        .filter_module("alex", log::LevelFilter::Debug)
        .filter_level(log::LevelFilter::Off)
        .init();

    let cli_args = cli::CliArgs::parse();

    if let cli::SubCommand::Daemon { root_folder_path } = cli_args.sub_command {
        match daemon::run(cli_args.port, root_folder_path).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to run daemon, {}", e);
            }
        }
        return;
    }

    let request = match cli::generate_request(&cli_args.sub_command) {
        Ok(out) => out,
        Err(e) => {
            log::error!("Failed to generate request from CLI arguments, {}", e);
            return;
        }
    };

    match connect(cli_args, request).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to communicate with daemon, {}", e);
            return;
        }
    };
}

async fn connect(cli_args: cli::CliArgs, request: protocol::Request) -> Result<(), error::Error> {
    let mut tcp_stream =
        match tokio::net::TcpStream::connect(format!("127.0.0.1:{}", cli_args.port)).await {
            Ok(out) => out,
            Err(e) => {
                // NOTE: if we were unable to connect to the daemon that means it is offline
                if e.kind() == std::io::ErrorKind::ConnectionRefused {
                    if cli_args.just_info {
                        print!("OFFLINE");
                    } else {
                        println!("{}", "> DAEMON is offline".red());
                    }
                    return Ok(());
                }
                return Err(error::Error::IOError(e.to_string()));
            }
        };
    tcp_stream.write_all(&request.to_bytes()).await?;

    // NOTE: this is important to ensure anyone reading from this stream does not loop
    // forever, it ensure EOF is reached on the "reader" side, by shutting down the "writer"
    tcp_stream.shutdown().await?;

    let response = protocol::Response::from_stream(&mut tcp_stream).await?;
    handle_response(cli_args, response).await?;

    Ok(())
}

async fn handle_response(
    cli_args: cli::CliArgs,
    response: protocol::Response,
) -> Result<(), error::Error> {
    match response {
        protocol::Response::PlaybackStarted { title } => {
            println!(
                "> Playing {quote}{}{quote}",
                title.purple(),
                quote = "\"".purple()
            );
        }
        protocol::Response::Next { playing_audio } => {
            println!("> Playing {}", playing_audio.purple());
        }
        protocol::Response::Paused => {
            println!("> Pausing playback")
        }
        protocol::Response::Resumed => {
            println!("> Resuming playback");
        }
        protocol::Response::Cleared => {
            println!("> Clearing player queue");
        }
        protocol::Response::Reloaded => {
            println!("> Player reloaded");
        }
        protocol::Response::SearchResults(search_results) => {
            for item in search_results.iter() {
                println!("-> {}", item.id);
            }
        }
        protocol::Response::StatusData(status_data) => {
            handle_status_response(cli_args, status_data)?
        }
        protocol::Response::ERROR { message } => {
            return Err(error::Error::ProtocolError(message));
        }
    }
    Ok(())
}

fn handle_status_response(
    cli_args: cli::CliArgs,
    status_data: protocol::StatusData,
) -> Result<(), error::Error> {
    if let cli::SubCommand::Status { sub_command } = cli_args.sub_command {
        match sub_command {
            Some(cli::StatusSubCommand::CurrentAudio) => {
                // NOTE: Here we go through all possibilities of (Option<String>, bool)
                match (status_data.current_audio, cli_args.just_info) {
                    (Some(current_audio), true) => print!("{}", current_audio),
                    (Some(current_audio), false) => {
                        println!("> Current Audio: {}", current_audio.purple())
                    }
                    (None, true) => print!("NO AUDIO"),
                    (None, false) => println!("> No audio is playing"),
                }
            }
            Some(cli::StatusSubCommand::IsPaused) => match cli_args.just_info {
                true => print!("{}", status_data.is_paused),
                false => println!("> {}", status_data.is_paused),
            },
            Some(cli::StatusSubCommand::IsQueueEmpty) => match cli_args.just_info {
                true => print!("{}", status_data.is_queue_empty),
                false => println!("> {}", status_data.is_queue_empty),
            },
            Some(cli::StatusSubCommand::Queue) => match cli_args.just_info {
                true => print!("{:?}", status_data.queue),
                false => println!("> {:#?}", status_data.queue),
            },
            None => match cli_args.just_info {
                true => print!("{}", serde_json::to_string(&status_data)?),
                false => println!("> {:#?}", status_data),
            },
        }
    } else {
        return Err(error::Error::ProtocolError(
            "Request and Response structure do not match".to_string(),
        ));
    }

    Ok(())
}
