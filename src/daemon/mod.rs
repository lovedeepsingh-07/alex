mod command;
mod player;

use crate::{constants, error};
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub struct Daemon {
    pub sender: crossbeam::channel::Sender<command::Command>,
    pub receiver: crossbeam::channel::Receiver<command::Command>,
}
impl Daemon {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::bounded::<command::Command>(constants::COMMAND_CAP);
        Daemon {
            sender: tx,
            receiver: rx,
        }
    }
    pub async fn run(&self) -> Result<(), error::Error> {
        let handler_rx = self.receiver.clone();
        tokio::spawn(async move {
            let mut player = player::Player::new();
            loop {
                match handler_rx.try_recv() {
                    Ok(cmd) => command::handle_command(cmd, &mut player),
                    Err(crossbeam::channel::TryRecvError::Empty) => {}
                    Err(crossbeam::channel::TryRecvError::Disconnected) => {
                        log::error!("the channel is disconnected");
                    }
                }
            }
        });

        let listener = tokio::net::TcpListener::bind(constants::SERVER_ADDRESS).await?;
        loop {
            let (mut tcp_stream, _) = listener.accept().await?;
            let mut reader = tokio::io::BufReader::new(&mut tcp_stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;

            let line_parts = line.split(":").collect::<Vec<&str>>();
            match line_parts[0] {
                "PLAYER" => {
                    if line_parts.len() != 2 {
                        return Err(error::Error::ParseError("invalid protocol structure".to_string()))
                    }
                    match line_parts[1] {
                        "PLAY" => self.sender.send(command::Command::PlayerPlay).unwrap(),
                        "PAUSE" => self.sender.send(command::Command::PlayerPause).unwrap(),
                        "RESUME" => self.sender.send(command::Command::PlayerResume).unwrap(),
                        "CLEAR" => self.sender.send(command::Command::PlayerClear).unwrap(),
                        _ => return Err(error::Error::ParseError("invalid protocol structure".to_string()))
                    }
                }
                _ => return Err(error::Error::ParseError("invalid protocol structure".to_string()))
            }
        }
    }
}
