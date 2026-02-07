use crate::{command, error, player, protocol};
use colored::Colorize;
use tokio::io::AsyncWriteExt;

pub async fn run(server_port: u16) -> Result<(), error::Error> {
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", server_port)).await?;
    let mut player = player::Player::new()?;

    log::info!("daemon running on {}", format!(":{}", server_port).blue());

    loop {
        let (mut tcp_stream, _) = listener.accept().await?;
        player.update_state()?;

        let request = protocol::Request::from_stream(&mut tcp_stream).await?;
        let cmd = request.to_cmd()?;

        let response = match command::handle(cmd, &mut player).await {
            Ok(out) => out,
            Err(_) => {
                let response = protocol::Response {
                    data: vec!["ERROR".to_string(), "NO_RESPONSE".to_string()],
                };
                response
            }
        };

        tcp_stream.write_all(&response.to_bytes()).await?;
        // NOTE: checkout the `main.rs` file for note regarding why this is here
        tcp_stream.shutdown().await?;
    }
}
