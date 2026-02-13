use crate::error;
use tokio::io::AsyncReadExt;

#[derive(Debug, bitcode::Encode, bitcode::Decode, serde::Serialize, serde::Deserialize)]
pub struct StatusData {
    pub current_audio: Option<String>,
    pub is_paused: bool,
    pub is_queue_empty: bool,
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum Response {
    PlaybackStarted { audio_label: String },
    Paused,
    Resumed,
    Cleared,
    Reloaded,
    SearchResults(Vec<String>),
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
