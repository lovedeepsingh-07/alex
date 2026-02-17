pub mod server;

use crate::error;

#[derive(Debug)]
pub enum TaskResult {
    Completed,
    Shutdown,
    Failed(error::Error),
}
