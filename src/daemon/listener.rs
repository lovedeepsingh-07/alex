use crate::{
    constants, daemon, error,
    protocol::{request, response},
};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

pub async fn run(handle_tx: mpsc::Sender<daemon::CR_Handle>) -> Result<(), error::Error> {
    let listener = tokio::net::TcpListener::bind(constants::SERVER_ADDRESS).await?;

    loop {
        let (mut tcp_stream, _) = listener.accept().await?;
        let request = request::Request::from_stream(&mut tcp_stream).await?;

        let cmd = request.to_cmd()?;
        let (response_tx, mut response_rx) = mpsc::channel::<response::Response>(1);
        handle_tx
            .send(daemon::CR_Handle {
                command: cmd,
                response_tx,
            })
            .await?;

        let response = match response_rx.recv().await {
            Some(out) => out,
            None => {
                tcp_stream
                    .write_all(
                        vec!["ERROR".to_string(), "NO_RESPONSE".to_string()]
                            .join("\n")
                            .as_bytes(),
                    )
                    .await?;
                continue;
            }
        };
        tcp_stream
            .write_all(response.data.join("\n").as_bytes())
            .await?;
        tcp_stream.shutdown().await?;
    }
}
