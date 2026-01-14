use crate::{constants, daemon::command, error};
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
    pub async fn read(
        &mut self,
        tcp_stream: &mut tokio::net::TcpStream,
    ) -> Result<(), error::Error> {
        let reader = tokio::io::BufReader::new(tcp_stream);
        let mut lines = reader.lines();
        while let Some(curr_line) = lines.next_line().await? {
            self.data.push(curr_line);
        }
        Ok(())
    }
    pub fn to_cmd(&self) -> Result<command::Command, error::Error> {
        let total_lines = self.data.len();
        if total_lines == 0 {
            return Err(error::Error::ParseError(
                "invalid protocol structure, the parsed request has no data".to_string(),
            ));
        }

        let private_key_hash = self
            .data
            .get(0)
            .ok_or_else(|| {
                error::Error::ParseError(
                    "invalid protocol structure, missing the private key hash".to_string(),
                )
            })?
            .to_string();
        if private_key_hash != self.private_key_hash {
            return Err(error::Error::ParseError(
                "invalid protocol structure, incorrect private key hash".to_string(),
            ));
        }

        let command_line = self.data.get(1).ok_or_else(|| {
            error::Error::ParseError(
                "invalid protocol structure, missing the command line".to_string(),
            )
        })?;
        match command_line.as_str() {
            "RELOAD" => return Ok(command::Command::Reload),
            "SEARCH" => {
                if let Some(search_term) = self.data.get(2) {
                    if search_term.trim().len() != 0 {
                        return Ok(command::Command::Search(Some(search_term.to_string())));
                    }
                    return Ok(command::Command::Search(Some(search_term.to_string())));
                }
                return Ok(command::Command::Search(None));
            }
            "PLAYER" => {
                let player_line = self.data.get(2).ok_or_else(|| {
                    error::Error::ParseError(
                        "invalid protocol structure, missing the player line".to_string(),
                    )
                })?;
                match player_line.as_str() {
                    "PLAY" => {
                        let audio_label = self.data.get(3).ok_or_else(|| {
                            error::Error::ParseError(
                                "invalid protocol structure, missing the player line".to_string(),
                            )
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
                        return Err(error::Error::ParseError(
                            "invalid protocol structure".to_string(),
                        ));
                    }
                }
            }
            _ => {
                return Err(error::Error::ParseError(
                    "invalid protocol structure".to_string(),
                ));
            }
        }
    }
}
