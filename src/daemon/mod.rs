pub mod command_handler;
pub mod listener;

use crate::{command, constants, error, protocol::response};
use tokio::sync::mpsc;

// Command-Response handle
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct CR_Handle {
    pub command: command::Command,
    pub response_tx: mpsc::Sender<response::Response>,
}

pub async fn run() -> Result<(), error::Error> {
    let (handle_tx, handle_rx) = mpsc::channel::<CR_Handle>(constants::COMMAND_CAP);
    command_handler::run(handle_rx).await?;
    listener::run(handle_tx).await?;
    Ok(())
}
