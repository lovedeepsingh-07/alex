pub mod cli;
pub mod command;
pub mod constants;
pub mod daemon;
pub mod error;
mod flatbuffers_gen;
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
    let request = cli::generate_request(cli_args.sub_command)?;

    let mut tcp_stream =
        match tokio::net::TcpStream::connect(format!("127.0.0.1:{}", cli_args.port)).await {
            Ok(out) => out,
            Err(e) => {
                // NOTE: if we were unable to connect to the daemon that means it is offline
                if e.kind() == std::io::ErrorKind::ConnectionRefused {
                    print!("OFFLINE");
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
    handle_response(response, cli_args.just_info).await?;

    Ok(())
}

async fn handle_response(
    response: protocol::Response,
    just_info: bool,
) -> Result<(), error::Error> {
    let response_packet: protocol::R_Packet = bitcode::decode(&response.packet)?;

    match response_packet.result {
        protocol::R_Result::OK => {},
        protocol::R_Result::ERROR { error_message } => {
            return Err(error::Error::ProtocolError(error_message));
        },
    }

    match response_packet.command {
        protocol::R_Command::Status { sub_command } => {
            match sub_command {
                protocol::R_StatusSubCommand::ALL { output } => {
                    if just_info {
                        print!("{}", output);
                    } else {
                        println!("> {}", output);
                    }
                },
                protocol::R_StatusSubCommand::CurrentAudio { output } => {
                    if let Some(current_audio) = output {
                        if just_info {
                            print!("{}", current_audio);
                        } else {
                            println!("> Current Audio: {}", current_audio.purple());
                        }
                    } else {
                        if just_info {
                            print!("NO AUDIO");
                        } else {
                            println!("> No audio is playing");
                        }
                    }
                },
                protocol::R_StatusSubCommand::IsPaused { output } => {
                    if just_info {
                        print!("{}", output);
                    } else {
                        println!("> {}", output);
                    }
                },
                protocol::R_StatusSubCommand::IsQueueEmpty { output } => {
                    if just_info {
                        print!("{}", output);
                    } else {
                        println!("> {}", output);
                    }
                },
            }
        },
        protocol::R_Command::Reload => {
            println!("> Player reloaded");
        },
        protocol::R_Command::Search { search_result } => {
            let mut audio_label_iter = search_result.iter();
            while let Some(audio_label) = audio_label_iter.next() {
                println!("-> {}", audio_label);
            }
        },
        protocol::R_Command::Player { sub_command } => {
            match sub_command {
                protocol::R_PlayerSubCommand::Play { audio_label } => {
                    println!("> Playing {}", audio_label.purple());
                },
                protocol::R_PlayerSubCommand::Pause => {
                    println!("> Resuming playback")
                },
                protocol::R_PlayerSubCommand::Resume => {
                    println!("> Pausing playback");
                },
                protocol::R_PlayerSubCommand::Clear => {
                    println!("> Clearing player queue");
                },
            }
        },
    }

    Ok(())
}
