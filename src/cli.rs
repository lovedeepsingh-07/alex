use crate::{constants, error, protocol};

#[derive(Debug, clap::Parser)]
#[command(version)]
pub struct CliArgs {
    #[arg(long, default_value=constants::DEFAULT_SERVER_PORT, global=true)]
    /// Server port
    pub port: u16,
    #[arg(long, global = true)]
    /// Pass this flag when calling the "status" subcommands from another program
    pub just_info: bool,
    #[command(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum SubCommand {
    /// Start the daemon by providing the path to your songs folder
    Daemon { root_folder_path: String },
    /// Get information such as which song is playing, whether playback is paused or not etc
    Status {
        #[command(subcommand)]
        sub_command: Option<StatusSubCommand>,
    },
    /// Reload the audio index to reflect any changes to the folder
    Reload,
    /// Search through the audio index
    Search { search_term: Option<String> },
    /// Play an audio [audio path (local) or id (from daemon's index)]
    Play { input: String },
    /// Push an audio to the playing queue [audio path (local) or id (from daemon's index)]
    Push {
        input: String,
        /// Pass this flag if you want the song to be pushed right after the currently playing song
        #[arg(long)]
        next: bool,
    },
    /// Skip to the next song in the playing queue
    Next,
    /// Pause playback (does nothing if already paused)
    Pause,
    /// Resume playback (does nothing if already resumed)
    Resume,
    /// Clear playing queue
    Clear,
}

#[derive(Debug, clap::Subcommand, PartialEq)]
pub enum StatusSubCommand {
    /// Current playing audio
    CurrentAudio,
    /// Is the playback paused ?
    IsPaused,
    /// Is the playing queue empty ?
    IsQueueEmpty,
}

pub fn generate_request(sub_command: &SubCommand) -> Result<protocol::Request, error::Error> {
    match sub_command {
        SubCommand::Daemon { root_folder_path: _ } => {}
        SubCommand::Status { sub_command: _ } => {
            return Ok(protocol::Request::Status);
        }
        SubCommand::Reload => {
            return Ok(protocol::Request::Reload);
        }
        SubCommand::Search { search_term } => {
            return Ok(protocol::Request::Search {
                search_term: search_term.clone(),
            });
        }
        SubCommand::Play { input } => {
            let input_path = std::path::Path::new(input);
            if let Ok(path_exists) = std::fs::exists(input_path) {
                if path_exists {
                    let abs_path = std::fs::canonicalize(input_path)?;
                    let input_path = abs_path.to_string_lossy().to_string();
                    return Ok(protocol::Request::Player {
                        sub_command: protocol::PlayerSubCommand::Play {
                            input: protocol::AudioInput {
                                id: input_path,
                                is_path: true,
                            },
                        },
                    });
                }
            }
            let input = input.trim().to_string();
            if input.len() == 0 {
                return Err(error::Error::InvalidInputError(
                    "You must provide a input with the play command".to_string(),
                ));
            }
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Play {
                    input: protocol::AudioInput {
                        id: input,
                        is_path: false,
                    }
                },
            });
        }
        SubCommand::Push { input, next } => {
            let input_path = std::path::Path::new(input);
            if let Ok(path_exists) = std::fs::exists(input_path) {
                if path_exists {
                    let abs_path = std::fs::canonicalize(input_path)?;
                    let input_path = abs_path.to_string_lossy().to_string();
                    return Ok(protocol::Request::Player {
                        sub_command: protocol::PlayerSubCommand::Push {
                            input: protocol::AudioInput {
                                id: input_path,
                                is_path: true,
                            },
                            next: next.clone(),
                        },
                    });
                }
            }
            let input = input.trim().to_string();
            if input.len() == 0 {
                return Err(error::Error::InvalidInputError(
                    "You must provide a input with the push command".to_string(),
                ));
            }
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Push {
                    input: protocol::AudioInput {
                        id: input,
                        is_path: false,
                    },
                    next: next.clone(),
                },
            });
        }
        SubCommand::Next => {
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Next,
            });
        }
        SubCommand::Pause => {
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Pause,
            });
        }
        SubCommand::Resume => {
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Resume,
            });
        }
        SubCommand::Clear => {
            return Ok(protocol::Request::Player {
                sub_command: protocol::PlayerSubCommand::Clear,
            });
        }
    }
    return Err(error::Error::IOError(
        "Failed to correctly parse CLI arguments".to_string(),
    ));
}
