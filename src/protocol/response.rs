use crate::error;
use std::collections::VecDeque;
use tokio::io::AsyncReadExt;

#[derive(Debug, bitcode::Encode, bitcode::Decode, serde::Serialize, serde::Deserialize)]
pub struct StatusData {
    pub current_audio: Option<String>,
    pub is_paused: bool,
    pub is_queue_empty: bool,
    pub queue: VecDeque<String>,
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct SearchResult {
    pub slug: String,
    pub score: f64,
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum Response {
    PlaybackStarted { input: String },
    Next { playing_audio: String },
    Paused,
    Resumed,
    Cleared,
    Reloaded,
    SearchResults(Vec<SearchResult>),
    StatusData(StatusData),
    ERROR { message: String },
}

impl Response {
    pub async fn from_stream(tcp_stream: &mut tokio::net::TcpStream) -> Result<Self, error::Error> {
        let mut buf: Vec<u8> = Vec::new();
        tcp_stream.read_to_end(&mut buf).await?;
        let value: Self = bitcode::decode(&buf)?;
        Ok(value)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        bitcode::encode(self)
    }
}
