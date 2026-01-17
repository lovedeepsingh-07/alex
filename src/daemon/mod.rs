pub mod command;
pub mod player;

use crate::{constants, error, request, response};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct CommandResponseHandle {
    command: command::Command,
    response_tx: mpsc::Sender<response::Response>,
}
pub async fn run() -> Result<(), error::Error> {
    let (handle_tx, mut handle_rx) = mpsc::channel::<CommandResponseHandle>(constants::COMMAND_CAP);
    tokio::spawn(async move {
        let mut player = match player::Player::new() {
            Ok(out) => out,
            Err(e) => {
                log::error!("{}", e.to_string());
                return;
            }
        };
        loop {
            match handle_rx.recv().await {
                Some(cmd_response_handle) => {
                    match command::handle(cmd_response_handle, &mut player).await {
                        Ok(_) => {}
                        Err(e) => {
                            log::error!("Failed to handle command: {}", e.to_string());
                        }
                    }
                }
                None => {
                    log::error!("Failed to receive command, the channel is disconnected");
                    break;
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
        let (response_tx, mut response_rx) = mpsc::channel::<response::Response>(1);
        handle_tx
            .send(CommandResponseHandle {
                command: cmd,
                response_tx,
            })
            .await?;
        let response = match response_rx.recv().await {
            Some(out) => out,
            None => {
                tcp_stream
                    .write_all(
                        vec!["ERROR".to_string(), "NO_RESPONSE".to_string()]
                            .join("\n")
                            .as_bytes(),
                    )
                    .await?;
                continue;
            }
        };
        tcp_stream
            .write_all(response.data.join("\n").as_bytes())
            .await?;
        // NOTE: this is important to ensure anyone reading from this stream does not loop
        // forever, it ensure EOF is reached on the reader side, by shutting down the writer
        tcp_stream.shutdown().await?;
    }
}
