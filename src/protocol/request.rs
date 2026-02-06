use crate::{command, constants, error};
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub struct Request {
    pub data: Vec<String>,
    pub private_key_hash: String,
}

impl Request {
    pub fn new() -> Self {
        Request {
            private_key_hash: blake3::hash(constants::PRIVATE_KEY.as_bytes()).to_string(),
            data: Vec::new(),
        }
    }
    pub async fn from_stream(
        tcp_stream: &mut tokio::net::TcpStream,
    ) -> Result<Self, error::Error> {
        let mut value = Self::new();
        let reader = tokio::io::BufReader::new(tcp_stream);
        let mut lines = reader.lines();
        while let Some(curr_line) = lines.next_line().await? {
            value.data.push(curr_line);
        }
        Ok(value)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.join("\n").into_bytes()
    }
    pub fn to_cmd(&self) -> Result<command::Command, error::Error> {
        let total_lines = self.data.len();
        if total_lines == 0 {
            return Err(error::Error::ProtocolError(
                "Parsed request has no data".to_string(),
            ));
        }

        let private_key_hash = self
            .data
            .get(0)
            .ok_or_else(|| error::Error::ProtocolError("Missing the private key hash".to_string()))?
            .to_string();
        if private_key_hash != self.private_key_hash {
            return Err(error::Error::ProtocolError(
                "Incorrect private key hash".to_string(),
            ));
        }

        let command_line = self
            .data
            .get(1)
            .ok_or_else(|| error::Error::ProtocolError("Missing the command line".to_string()))?;
        match command_line.as_str() {
            "STATUS" => {
                if let Some(status_sub_command) = self.data.get(2) {
                    let sub_command = match status_sub_command.as_str() {
                        "CURRENT_AUDIO" => command::StatusSubCommand::CurrentAudio,
                        "IS_PAUSED" => command::StatusSubCommand::IsPaused,
                        "IS_QUEUE_EMPTY" => command::StatusSubCommand::IsQueueEmpty,
                        _ => {
                            return Err(error::Error::ProtocolError(
                                "Invalid player sub-command".to_string(),
                            ));
                        }
                    };
                    return Ok(command::Command::Status {
                        sub_command: Some(sub_command),
                    });
                }
                return Ok(command::Command::Status { sub_command: None });
            }
            "RELOAD" => return Ok(command::Command::Reload),
            "SEARCH" => {
                if let Some(search_term) = self.data.get(2) {
                    if search_term.trim().len() != 0 {
                        return Ok(command::Command::Search {
                            search_term: Some(search_term.to_string()),
                        });
                    }
                    return Ok(command::Command::Search { search_term: None });
                }
                return Ok(command::Command::Search { search_term: None });
            }
            "PLAYER" => {
                let player_line = self.data.get(2).ok_or_else(|| {
                    error::Error::ProtocolError("Missing the player line".to_string())
                })?;
                match player_line.as_str() {
                    "PLAY" => {
                        let audio_label = self.data.get(3).ok_or_else(|| {
                            error::Error::ProtocolError("Missing the player line".to_string())
                        })?;
                        return Ok(command::Command::Player {
                            sub_command: command::PlayerSubCommand::Play {
                                audio_label: audio_label.clone(),
                            },
                        });
                    }
                    "PAUSE" => {
                        return Ok(command::Command::Player {
                            sub_command: command::PlayerSubCommand::Pause,
                        });
                    }
                    "RESUME" => {
                        return Ok(command::Command::Player {
                            sub_command: command::PlayerSubCommand::Resume,
                        });
                    }
                    "CLEAR" => {
                        return Ok(command::Command::Player {
                            sub_command: command::PlayerSubCommand::Clear,
                        });
                    }
                    _ => {
                        return Err(error::Error::ProtocolError(
                            "Invalid player sub-command".to_string(),
                        ));
                    }
                }
            }
            _ => {
                return Err(error::Error::ProtocolError("Invalid command".to_string()));
            }
        }
    }
}
