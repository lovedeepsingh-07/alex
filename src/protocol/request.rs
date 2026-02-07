use crate::{command, error, flatbuffers_gen::request_packet_ as request_packet};
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct Request {
    pub packet: Vec<u8>,
}

impl Request {
    pub fn new() -> Self {
        Request { packet: Vec::new() }
    }
    pub async fn from_stream(tcp_stream: &mut tokio::net::TcpStream) -> Result<Self, error::Error> {
        let mut value = Self::new();
        tcp_stream.read_to_end(&mut value.packet).await?;
        Ok(value)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.packet.clone()
    }
    pub fn to_cmd(&self) -> Result<command::Command, error::Error> {
        let request_packet = request_packet::root_as_request_packet(&self.packet)?;

        match request_packet.command_type() {
            request_packet::Command::Status => {
                if let Some(status_command) = request_packet.command_as_status() {
                    let status_sub_command = status_command.sub_command();
                    match status_sub_command {
                        request_packet::StatusSubCommand::CurrentAudio => {
                            return Ok(command::Command::Status {
                                sub_command: Some(command::StatusSubCommand::CurrentAudio),
                            });
                        }
                        request_packet::StatusSubCommand::IsPaused => {
                            return Ok(command::Command::Status {
                                sub_command: Some(command::StatusSubCommand::IsPaused),
                            });
                        }
                        request_packet::StatusSubCommand::IsQueueEmpty => {
                            return Ok(command::Command::Status {
                                sub_command: Some(command::StatusSubCommand::IsQueueEmpty),
                            });
                        }
                        request_packet::StatusSubCommand::NONE => {
                            return Ok(command::Command::Status { sub_command: None });
                        }
                        _ => {
                            return Err(error::Error::ProtocolError(
                                "Invalid request packet status sub command".to_string(),
                            ));
                        }
                    }
                }
                return Err(error::Error::ProtocolError(
                    "Failed to get request packet as Status command".to_string(),
                ));
            }
            request_packet::Command::Reload => {
                return Ok(command::Command::Reload);
            }
            request_packet::Command::Search => {
                if let Some(search_command) = request_packet.command_as_search() {
                    let search_term = match search_command.search_term() {
                        Some(search_term) => {
                            let search_term = search_term.trim().to_string();
                            if search_term.len() == 0 {
                                None
                            } else {
                                Some(search_term)
                            }
                        }
                        None => None,
                    };
                    return Ok(command::Command::Search { search_term });
                }
                return Err(error::Error::ProtocolError(
                    "Failed to get request packet as Search command".to_string(),
                ));
            }
            request_packet::Command::Player => {
                if let Some(player_command) = request_packet.command_as_player() {
                    match player_command.sub_command_type() {
                        request_packet::PlayerSubCommand::Play => {
                            if let Some(sub_command_play) = player_command.sub_command_as_play() {
                                if let Some(audio_label) = sub_command_play.audio_label() {
                                    return Ok(command::Command::Player {
                                        sub_command: command::PlayerSubCommand::Play {
                                            audio_label: audio_label.to_string(),
                                        },
                                    });
                                }
                                return Err(error::Error::ProtocolError(
                                    "Failed to get the audio label for Play sub command"
                                        .to_string(),
                                ));
                            }
                            return Err(error::Error::ProtocolError(
                                "Failed to get player sub command as Play".to_string(),
                            ));
                        }
                        request_packet::PlayerSubCommand::Pause => {
                            if let Some(_) = player_command.sub_command_as_pause() {
                                return Ok(command::Command::Player {
                                    sub_command: command::PlayerSubCommand::Pause,
                                });
                            }
                            return Err(error::Error::ProtocolError(
                                "Failed to get player sub command as Pause".to_string(),
                            ));
                        }
                        request_packet::PlayerSubCommand::Resume => {
                            if let Some(_) = player_command.sub_command_as_resume() {
                                return Ok(command::Command::Player {
                                    sub_command: command::PlayerSubCommand::Resume,
                                });
                            }
                            return Err(error::Error::ProtocolError(
                                "Failed to get player sub command as Resume".to_string(),
                            ));
                        }
                        request_packet::PlayerSubCommand::Clear => {
                            if let Some(_) = player_command.sub_command_as_clear() {
                                return Ok(command::Command::Player {
                                    sub_command: command::PlayerSubCommand::Clear,
                                });
                            }
                            return Err(error::Error::ProtocolError(
                                "Failed to get player sub command as Clear".to_string(),
                            ));
                        }
                        _ => {
                            return Err(error::Error::ProtocolError(
                                "Invalid player sub command type".to_string(),
                            ));
                        }
                    }
                }
                return Err(error::Error::ProtocolError(
                    "Failed to get request packet as Player command".to_string(),
                ));
            }
            _ => {
                return Err(error::Error::ProtocolError(
                    "Invalid request packet command type".to_string(),
                ));
            }
        }
    }
}
