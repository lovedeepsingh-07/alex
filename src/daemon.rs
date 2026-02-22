use crate::{constants, error, protocol};
use colored::Colorize;
use tokio::io::AsyncWriteExt;

pub async fn run(server_port: u16, root_folder_path: String) -> Result<(), error::Error> {
    let root_folder_path = std::path::Path::new(root_folder_path.as_str());
    if !root_folder_path.exists() {
        return Err(error::Error::InvalidInputError(
            "Path provided to the daemon DOES_NOT exist".to_string(),
        ));
    }
    let root_folder_path = std::fs::canonicalize(root_folder_path)?;
    if !root_folder_path.is_dir() {
        return Err(error::Error::InvalidInputError(
            "Path provided to the daemon IS_NOT a valid folder".to_string(),
        ));
    }
    // let mut player = player::Player::new(root_folder_path)?;

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", server_port)).await?;
    log::info!("daemon running on {}", format!(":{}", server_port).blue());

    loop {
        tokio::select! {
            conn = listener.accept() => {
                let (mut tcp_stream, _) = conn?;
                let request = protocol::Request::from_stream(&mut tcp_stream).await?;
                log::info!("{:#?}", request);
                // let response = handlers::handle(request, &mut player);
                // tcp_stream.write_all(&response.to_bytes()).await?;
                // NOTE: checkout the `main.rs` file for note regarding why this is here
                tcp_stream.shutdown().await?;
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(constants::DELTA_TIME_MS)) => {
                // player.tick(|next_audio_title| {})?;
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
