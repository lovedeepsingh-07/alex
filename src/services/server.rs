use crate::{error, handlers, player, protocol, services};
use colored::Colorize;
use tokio::{io::AsyncWriteExt, sync::watch, task::JoinSet};

#[derive(Debug)]
pub struct ServerProps<'a> {
    pub tasks: &'a mut JoinSet<services::TaskResult>,
    pub shutdown_rx: watch::Receiver<bool>,
    pub server_port: u16,
    pub folder_path: String,
}

pub fn spawn_task<'a>(props: ServerProps<'a>) {
    let ServerProps {
        tasks,
        mut shutdown_rx,
        server_port,
        folder_path,
    } = props;
    tasks.spawn(async move {
        tokio::select! {
            result = run_service(server_port, folder_path) => {
                match result {
                    Ok(_) => services::TaskResult::Completed,
                    Err(e) => services::TaskResult::Failed(e)
                }
            }
            _ = shutdown_rx.wait_for(|value| *value == true) => services::TaskResult::Shutdown,
        }
    });
}

async fn run_service(server_port: u16, folder_path: String) -> Result<(), error::Error> {
    let folder_path = std::path::PathBuf::from(folder_path);
    let abs_folder_path = std::fs::canonicalize(folder_path)?;
    if !abs_folder_path.exists() {
        return Err(error::Error::InvalidInputError(
            "Path provided to the daemon DOES_NOT exist".to_string(),
        ));
    }
    if !abs_folder_path.is_dir() || abs_folder_path.is_file() {
        return Err(error::Error::InvalidInputError(
            "Path provided to the daemon IS_NOT a valid folder".to_string(),
        ));
    }

    let listener = tokio::net::TcpListener::bind(("127.0.0.1", server_port)).await?;
    let mut player = player::Player::new(abs_folder_path)?;

    log::info!("daemon running on {}", format!(":{}", server_port).blue());

    loop {
        let (mut tcp_stream, _) = listener.accept().await?;
        let request = protocol::Request::from_stream(&mut tcp_stream).await?;
        let response = handlers::handle(request, &mut player).await;
        tcp_stream.write_all(&response.to_bytes()).await?;
        // NOTE: checkout the `main.rs` file for note regarding why this is here
        tcp_stream.shutdown().await?;
    }
}
