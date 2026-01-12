pub mod command;
pub mod player;

use crate::{constants, error, request};

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
                    Ok(cmd) => command::handle(cmd, &mut player),
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
            let mut request = request::Request::new();
            request.read(&mut tcp_stream).await?;
            let cmd = request.to_cmd()?;
            self.sender.send(cmd).unwrap();
        }
    }
}
