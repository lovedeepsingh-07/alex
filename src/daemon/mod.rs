pub mod command;
pub mod player;

use crate::{constants, error, request, response};
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct CommandResponseHandle {
    command: command::Command,
    response_sender: crossbeam::channel::Sender<response::Response>,
}
#[derive(Debug)]
pub struct Daemon {
    pub sender: crossbeam::channel::Sender<CommandResponseHandle>,
    pub receiver: crossbeam::channel::Receiver<CommandResponseHandle>,
}
impl Daemon {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::bounded::<CommandResponseHandle>(constants::COMMAND_CAP);
        Daemon {
            sender: tx,
            receiver: rx,
        }
    }
    pub async fn run(&self) -> Result<(), error::Error> {
        let handler_rx = self.receiver.clone();
        tokio::spawn(async move {
            let mut player = match player::Player::new() {
                Ok(out) => out,
                Err(e) => {
                    log::error!("{}", e.to_string());
                    std::process::exit(1);
                }
            };
            loop {
                match handler_rx.try_recv() {
                    Ok(cmd_response_handle) => match command::handle(cmd_response_handle, &mut player) {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Failed to handle command: {}", e.to_string());
                        }
                    },
                    Err(crossbeam::channel::TryRecvError::Empty) => {}
                    Err(crossbeam::channel::TryRecvError::Disconnected) => {
                        log::error!("Failed to receive command, the channel is disconnected");
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
            let (response_sender, response_receiver) = crossbeam::channel::unbounded::<response::Response>();
            self.sender.send(CommandResponseHandle { command: cmd, response_sender })?;
            let response = response_receiver.recv()?;
            tcp_stream
                .write_all(response.data.join("\n").as_bytes())
                .await?;
            // NOTE: this is important to ensure anyone reading from this stream does not loop
            // forever, it ensure EOF is reached on the reader side, by shutting down the writer
            tcp_stream.shutdown().await?;
        }
    }
}
