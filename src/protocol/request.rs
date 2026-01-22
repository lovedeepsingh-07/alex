use crate::{command, constants, error};
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub(crate) struct Request {
    pub(crate) data: Vec<String>,
    pub(crate) private_key_hash: String,
}

impl Request {
    pub(crate) fn new() -> Self {
        Request {
            private_key_hash: blake3::hash(constants::PRIVATE_KEY.as_bytes()).to_string(),
            data: Vec::new(),
        }
    }
    pub(crate) async fn from_stream(
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
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.data.join("\n").into_bytes()
    }
    pub(crate) fn to_cmd(&self) -> Result<command::Command, error::Error> {
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
            "STATUS" => return Ok(command::Command::Status),
            "RELOAD" => return Ok(command::Command::Reload),
            "SEARCH" => {
                if let Some(search_term) = self.data.get(2) {
                    if search_term.trim().len() != 0 {
                        return Ok(command::Command::Search(Some(search_term.to_string())));
                    }
                    return Ok(command::Command::Search(None));
                }
                return Ok(command::Command::Search(None));
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
                        return Ok(command::Command::Player(command::PlayerSubCommand::Play(
                            audio_label.clone(),
                        )));
                    }
                    "PAUSE" => {
                        return Ok(command::Command::Player(command::PlayerSubCommand::Pause));
                    }
                    "RESUME" => {
                        return Ok(command::Command::Player(command::PlayerSubCommand::Resume));
                    }
                    "CLEAR" => {
                        return Ok(command::Command::Player(command::PlayerSubCommand::Clear));
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
