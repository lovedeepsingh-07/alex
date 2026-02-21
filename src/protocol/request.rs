use crate::{error, player};
use tokio::io::AsyncReadExt;

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct AudioInput {
    pub id: player::AudioID,
    pub is_path: bool,
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum PlayerSubCommand {
    Play { input: AudioInput },
    Push {
        input: AudioInput,
        next: bool,
    },
    Next,
    Pause,
    Resume,
    Clear,
}

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub enum Request {
    Status,
    Reload,
    Search { search_term: Option<String> },
    Player { sub_command: PlayerSubCommand },
}

impl Request {
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
