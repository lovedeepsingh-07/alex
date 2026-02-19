pub mod playback;
pub mod server;

use crate::{error, protocol};
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct RequestCarrier {
    request: protocol::Request,
    response_tx: mpsc::Sender<protocol::Response>,
}

#[derive(Debug)]
pub enum TaskResult {
    Completed,
    Shutdown,
    Failed(error::Error),
}
