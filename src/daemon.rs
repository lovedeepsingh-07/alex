use crate::{error, services};
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
};

pub async fn run(server_port: u16, folder_path: String) -> Result<(), error::Error> {
    let mut tasks: JoinSet<services::TaskResult> = JoinSet::new();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let (request_carrier_tx, request_carrier_rx) = mpsc::channel(64);

    services::playback::spawn_task(services::playback::PlaybackProps {
        tasks: &mut tasks,
        shutdown_rx: shutdown_rx.clone(),
        folder_path,
        request_carrier_rx,
    }).await?;
    services::server::spawn_task(services::server::ServerProps {
        tasks: &mut tasks,
        shutdown_rx: shutdown_rx.clone(),
        server_port,
        request_carrier_tx,
    }).await?;

    tokio::select! {
        join_result = tasks.join_next() => {
            if let Some(result) = join_result {
                let _ = shutdown_tx.send(true);
                handle_task_result(result);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            let _ = shutdown_tx.send(true);
            log::info!("Shutting down...");
            while let Some(result) = tasks.join_next().await {
                handle_task_result(result);
            }
        }
    }
    Ok(())
}

fn handle_task_result(result: Result<services::TaskResult, tokio::task::JoinError>) {
    match result {
        Ok(services::TaskResult::Completed) => {
            log::error!("Task returned unexpectedly");
        },
        Ok(services::TaskResult::Shutdown) => {
            log::debug!("Shutting down Task");
        },
        Ok(services::TaskResult::Failed(e)) => {
            log::error!("Task failed with error, {}", e.to_string());
        },
        Err(e) => log::error!("Join error: {}", e),
    }
}
