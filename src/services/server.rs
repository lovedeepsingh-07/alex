use crate::{error, protocol, services};
use colored::Colorize;
use tokio::{
    io::AsyncWriteExt,
    sync::{mpsc, watch},
    task::JoinSet,
};

#[derive(Debug)]
pub struct ServerProps<'a> {
    pub tasks: &'a mut JoinSet<services::TaskResult>,
    pub shutdown_rx: watch::Receiver<bool>,
    pub server_port: u16,
    pub request_carrier_tx: mpsc::Sender<services::RequestCarrier>,
}

pub async fn spawn_task<'a>(props: ServerProps<'a>) -> Result<(), error::Error> {
    let ServerProps {
        tasks,
        mut shutdown_rx,
        server_port,
        request_carrier_tx,
    } = props;
    tasks.spawn(async move {
        tokio::select! {
            result = run_service(server_port, request_carrier_tx) => {
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
    server_port: u16,
    request_carrier_tx: mpsc::Sender<services::RequestCarrier>,
) -> Result<(), error::Error> {
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", server_port)).await?;
    log::info!("daemon running on {}", format!(":{}", server_port).blue());

    loop {
        let (mut tcp_stream, _) = listener.accept().await?;
        let (response_tx, mut response_rx) = mpsc::channel::<protocol::Response>(1);
        let request = protocol::Request::from_stream(&mut tcp_stream).await?;
        request_carrier_tx
            .send(services::RequestCarrier {
                request,
                response_tx,
            })
            .await?;
        let response = response_rx.recv().await.ok_or_else(|| {
            error::Error::ChannelReceiveError("Response channel is closed".to_string())
        })?;
        tcp_stream.write_all(&response.to_bytes()).await?;
        // NOTE: checkout the `main.rs` file for note regarding why this is here
        tcp_stream.shutdown().await?;
    }
}
