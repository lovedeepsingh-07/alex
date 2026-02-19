use crate::{error, handlers, player, services};
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
};

#[derive(Debug)]
pub struct PlaybackProps<'a> {
    pub tasks: &'a mut JoinSet<services::TaskResult>,
    pub shutdown_rx: watch::Receiver<bool>,
    pub folder_path: String,
    pub request_carrier_rx: mpsc::Receiver<services::RequestCarrier>,
}

pub async fn spawn_task<'a>(props: PlaybackProps<'a>) -> Result<(), error::Error> {
    let PlaybackProps {
        tasks,
        mut shutdown_rx,
        folder_path,
        request_carrier_rx,
    } = props;

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

    tasks.spawn(async move {
        tokio::select! {
            result = run_service(abs_folder_path, request_carrier_rx) => {
                match result {
                    Ok(_) => services::TaskResult::Completed,
                    Err(e) => services::TaskResult::Failed(e)
                }
            }
            _ = shutdown_rx.wait_for(|value| *value == true) => services::TaskResult::Shutdown,
        }
    });

    Ok(())
}

async fn run_service(
    folder_path: std::path::PathBuf,
    mut request_carrier_rx: mpsc::Receiver<services::RequestCarrier>,
) -> Result<(), error::Error> {
    let mut player = player::Player::new(folder_path)?;
    loop {
        tokio::select! {
            Some(carrier) = request_carrier_rx.recv() => {
                let response = handlers::handle(carrier.request, &mut player);
                carrier.response_tx.send(response).await?;
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                player.update_state()?;
            }
        }
    }
}
