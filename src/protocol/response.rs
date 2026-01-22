use crate::error;
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub(crate) struct Response {
    pub(crate) data: Vec<String>,
}

impl Response {
    pub(crate) fn new() -> Self {
        Response { data: Vec::new() }
    }
    pub(crate) async fn from_stream(
        tcp_stream: &mut tokio::net::TcpStream,
    ) -> Result<Self, error::Error> {
        let mut value = Self::new();
        let reader = tokio::io::BufReader::new(tcp_stream);
        let mut lines = reader.lines();
        while let Some(curr_line) = lines.next_line().await? {
            value.data.push(curr_line);
        }
        Ok(value)
    }
    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.data.join("\n").into_bytes()
    }
}
