use tokio::io::AsyncWriteExt;
use colored::Colorize;
use crate::{error, player, protocol, handlers};

pub async fn run(server_port: u16, folder_path: String) -> Result<(), error::Error> {
    let folder_path = std::path::Path::new(folder_path.as_str());
    if !folder_path.exists() {
        return Err(error::Error::InvalidInputError(
            "Path provided to the daemon DOES_NOT exist".to_string(),
        ));
    }
    let abs_folder_path = std::fs::canonicalize(folder_path)?;
    if !abs_folder_path.is_dir() {
        return Err(error::Error::InvalidInputError(
            "Path provided to the daemon IS_NOT a valid folder".to_string(),
        ));
    }
    let mut player = player::Player::new(abs_folder_path)?;

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", server_port)).await?;
    log::info!("daemon running on {}", format!(":{}", server_port).blue());

    loop {
        tokio::select!{
            conn = listener.accept() => {
                let (mut tcp_stream, _) = conn?;
                let request = protocol::Request::from_stream(&mut tcp_stream).await?;
                let response = handlers::handle(request, &mut player);
                tcp_stream.write_all(&response.to_bytes()).await?;
                // NOTE: checkout the `main.rs` file for note regarding why this is here
                tcp_stream.shutdown().await?;
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                player.update_state()?;
                continue;
            }
            _ = tokio::signal::ctrl_c() => {
                log::info!("Shutting down...");
                break;
            }
        };
    }
    Ok(())
}
