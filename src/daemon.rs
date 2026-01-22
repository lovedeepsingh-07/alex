use crate::{
    command, constants, error, player,
    protocol::{request, response},
};
use tokio::io::AsyncWriteExt;

pub(crate) async fn run() -> Result<(), error::Error> {
    let listener = tokio::net::TcpListener::bind(constants::SERVER_ADDRESS).await?;
    let mut player = player::Player::new()?;

    loop {
        let (mut tcp_stream, _) = listener.accept().await?;
        player.update_state()?;

        let request = request::Request::from_stream(&mut tcp_stream).await?;
        let cmd = request.to_cmd()?;

        let response = match command::handle(cmd, &mut player).await {
            Ok(out) => out,
            Err(_) => {
                let response = response::Response {
                    data: vec!["ERROR".to_string(), "NO_RESPONSE".to_string()],
                };
                response
            }
        };

        tcp_stream.write_all(&response.to_bytes()).await?;
        tcp_stream.shutdown().await?;
    }
}
