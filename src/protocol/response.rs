use crate::error;
use tokio::io::AsyncReadExt;

#[derive(Debug)]
pub struct Response {
    pub data: Vec<String>,
    pub packet: Vec<u8>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            data: Vec::new(),
            packet: Vec::new(),
        }
    }
    pub async fn from_stream(tcp_stream: &mut tokio::net::TcpStream) -> Result<Self, error::Error> {
        let mut value = Self::new();
        tcp_stream.read_to_end(&mut value.packet).await?;
        Ok(value)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.packet.clone()
    }
}
