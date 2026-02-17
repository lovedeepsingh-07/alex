use crate::{error, protocol, services};
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
};

#[derive(Debug)]
pub struct ResponseCarrier {
    pub request: protocol::Request,
    pub response_tx: mpsc::Sender<protocol::Response>,
}

pub async fn run(server_port: u16, folder_path: String) -> Result<(), error::Error> {
    let mut tasks: JoinSet<services::TaskResult> = JoinSet::new();
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    services::server::spawn_task(services::server::ServerProps {
        tasks: &mut tasks,
        shutdown_rx: shutdown_rx.clone(),
        server_port,
        folder_path,
    });

    tokio::select! {
        join_result = tasks.join_next() => {
            if let Some(join_result) = join_result {
                let _ = shutdown_tx.send(true);
                match join_result {
                    Ok(services::TaskResult::Completed) => {
                        log::error!("Task returned unexpectedly");
                    },
                    Ok(services::TaskResult::Shutdown) => {
                        log::debug!("Shutting down Task");
                    },
                    Ok(services::TaskResult::Failed(e)) => {
                        log::error!("Task failed with error, {}", e.to_string());
                    },
                    Err(e) => {
                        log::error!("Failed to join task, {}", e.to_string());
                    },
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            let _ = shutdown_tx.send(true);
            log::info!("Shutting down...");
            while let Some(res) = tasks.join_next().await {
                match res {
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
        }
    }
    Ok(())
}
