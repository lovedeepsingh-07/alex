use crate::error;
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub struct Response {
    pub data: Vec<String>
}

impl Response {
    pub fn new() -> Self {
        Response {
            data: Vec::new(),
        }
    }
    pub async fn read(&mut self, tcp_stream: &mut tokio::net::TcpStream) -> Result<(), error::Error> {
        let reader = tokio::io::BufReader::new(tcp_stream);
        let mut lines = reader.lines();
        while let Some(curr_line) = lines.next_line().await? {
            self.data.push(curr_line);
        }
        Ok(())
    }
}
