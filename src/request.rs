use crate::{cli, constants, daemon::command, error};
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct Request {
    pub data: Vec<String>,
}

impl Request {
    pub fn new() -> Self {
        Request { data: Vec::new() }
    }

    pub fn generate(&mut self, sub_command: cli::SubCommand) -> Result<(), error::Error> {
        self.data.push(String::new());
        match sub_command {
            cli::SubCommand::Daemon => {}
            cli::SubCommand::Reload => {
                self.data.push("RELOAD".to_string());
            }
            cli::SubCommand::Search => {
                self.data.push("SEARCH".to_string());
            }
            cli::SubCommand::Player { sub_command } => {
                self.data.push("PLAYER".to_string());
                match sub_command {
                    cli::PlayerSubCommand::Play { audio_label } => {
                        self.data.push("PLAY".to_string());
                        self.data.push(audio_label);
                    }
                    cli::PlayerSubCommand::Pause => {
                        self.data.push("PAUSE".to_string());
                    }
                    cli::PlayerSubCommand::Resume => {
                        self.data.push("RESUME".to_string());
                    }
                    cli::PlayerSubCommand::Clear => {
                        self.data.push("CLEAR".to_string());
                    }
                }
            }
        }
        Ok(())
    }
    pub async fn send(&self) -> Result<(), error::Error> {
        let mut tcp_stream = tokio::net::TcpStream::connect(constants::SERVER_ADDRESS).await?;
        tcp_stream
            .write_all(self.data.join("\n").as_bytes())
            .await?;
        Ok(())
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

        let command_line = self.data.get(1).ok_or_else(|| {
            error::Error::ParseError(
                "invalid protocol structure, missing the command line".to_string(),
            )
        })?;
        match command_line.as_str() {
            "RELOAD" => return Ok(command::Command::Reload),
            "SEARCH" => return Ok(command::Command::Search),
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
                        return Ok(command::Command::PlayerPlay(audio_label.clone()));
                    }
                    "PAUSE" => return Ok(command::Command::PlayerPause),
                    "RESUME" => return Ok(command::Command::PlayerResume),
                    "CLEAR" => return Ok(command::Command::PlayerClear),
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
// let line_parts = line.split(":").collect::<Vec<&str>>();
// match line_parts[0] {
//     "RELOAD" => self.sender.send(command::Command::Reload).unwrap(),
//     "SEARCH" => self.sender.send(command::Command::Search).unwrap(),
//     "PLAYER" => {
//         if line_parts.len() < 2 {
//             return Err(error::Error::ParseError(
//                 "invalid protocol structure".to_string(),
//             ));
//         }
//         match line_parts[1] {
//             "PLAY" => {
//                 let audio_label = line_parts.get(2).ok_or_else(|| {
//                     error::Error::ParseError("invalid protocol structure".to_string())
//                 })?;
//                 self.sender.send(command::Command::PlayerPlay(audio_label.to_string())).unwrap()
//             }
//             "PAUSE" => self.sender.send(command::Command::PlayerPause).unwrap(),
//             "RESUME" => self.sender.send(command::Command::PlayerResume).unwrap(),
//             "CLEAR" => self.sender.send(command::Command::PlayerClear).unwrap(),
//             _ => {
//                 return Err(error::Error::ParseError(
//                     "invalid protocol structure".to_string(),
//                 ));
//             }
//         }
//     }
//     _ => {
//         return Err(error::Error::ParseError(
//             "invalid protocol structure".to_string(),
//         ));
//     }
// }
