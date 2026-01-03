use crate::{constants, error};
use tokio::io::AsyncWriteExt;

pub async fn run() -> Result<(), error::Error> {
    let mut stream = tokio::net::TcpStream::connect(constants::SERVER_ADDRESS).await?;
    stream.write_all(b"STATUS").await?;
    Ok(())
}
