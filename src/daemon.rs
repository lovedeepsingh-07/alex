use crate::{error, handlers, player, protocol};
use colored::Colorize;
use tokio::io::AsyncWriteExt;

pub async fn run(server_port: u16, folder_path: String) -> Result<(), error::Error> {
    let folder_path = std::path::PathBuf::from(folder_path);
    let abs_folder_path = std::fs::canonicalize(folder_path)?;
    if !abs_folder_path.exists() {
        return Err(error::Error::InvalidInputError("Path provided to the daemon DOES_NOT exist".to_string()));
    }
    if !abs_folder_path.is_dir() || abs_folder_path.is_file() {
        return Err(error::Error::InvalidInputError("Path provided to the daemon IS_NOT a valid folder".to_string()));
    }

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", server_port)).await?;
    let mut player = player::Player::new(abs_folder_path)?;

    log::info!("daemon running on {}", format!(":{}", server_port).blue());

    loop {
        let (mut tcp_stream, _) = listener.accept().await?;
        player.update_state()?;

        let request = protocol::Request::from_stream(&mut tcp_stream).await?;
        let response = handlers::handle(request, &mut player).await;

        tcp_stream.write_all(&response.to_bytes()).await?;

        // NOTE: checkout the `main.rs` file for note regarding why this is here
        tcp_stream.shutdown().await?;
    }
}
