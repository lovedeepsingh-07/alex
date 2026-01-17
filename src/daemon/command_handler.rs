use crate::{command, daemon, error, player};
use tokio::sync::mpsc;

pub async fn run(mut handle_rx: mpsc::Receiver<daemon::CR_Handle>) -> Result<(), error::Error> {
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
                            break;
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
    Ok(())
}
