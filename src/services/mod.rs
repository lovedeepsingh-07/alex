pub mod server;
pub mod playback;

use tokio::sync::mpsc;
use crate::{error, protocol};

#[derive(Debug)]
pub struct RequestCarrier {
    request: protocol::Request,
    response_tx: mpsc::Sender<protocol::Response>
}

#[derive(Debug)]
pub enum TaskResult {
    Completed,
    Shutdown,
    Failed(error::Error),
}
