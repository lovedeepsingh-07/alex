pub mod cli;
pub mod constants;
pub mod daemon;
pub mod error;
pub mod handlers;
pub mod player;
pub mod protocol;

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

    if cli_args.sub_command == cli::SubCommand::Daemon {
        match daemon::run(cli_args.port).await {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to run daemon, {}", e.to_string());
            }
        }
        return;
    }

    match connect(cli_args).await {
        Ok(_) => {}
        Err(e) => {
            log::error!("Failed to make request, {}", e.to_string());
        }
    };
}

async fn connect(cli_args: cli::CliArgs) -> Result<(), error::Error> {
    let request = cli::generate_request(&cli_args.sub_command)?;

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
        protocol::Response::PlaybackStarted { audio_label } => {
            println!("> Playing {}", audio_label);
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
            let mut audio_label_iter = search_results.iter();
            while let Some(audio_label) = audio_label_iter.next() {
                println!("-> {}", audio_label);
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
            None => match cli_args.just_info {
                true => print!("{}", serde_json::to_string(&status_data)?),
                false => println!("> {}", serde_json::to_string(&status_data)?),
            },
        }
    } else {
        return Err(error::Error::ProtocolError(
            "Request and Response structure do not match".to_string(),
        ));
    }

    Ok(())
}
