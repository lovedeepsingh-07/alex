pub mod cli;
pub mod constants;
pub mod daemon;
pub mod error;
pub mod request;
pub mod response;

use clap::Parser;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let logger_env = env_logger::Env::default().filter_or("RUST_LOG", "alex=debug");
    env_logger::init_from_env(logger_env);

    let cli_args = cli::CliArgs::parse();
    if cli_args.sub_command == cli::SubCommand::Daemon {
        let daemon = daemon::Daemon::new();
        match daemon.run().await {
            Ok(_) => {}
            Err(e) => {
                log::error!("{}", e.to_string());
            }
        };
        return;
    }

    let request = match cli::generate_request(cli_args.sub_command) {
        Ok(out) => out,
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(1);
        }
    };
    match async {
        let mut tcp_stream = tokio::net::TcpStream::connect(constants::SERVER_ADDRESS).await?;
        tcp_stream
            .write_all(request.data.join("\n").as_bytes())
            .await?;
        // NOTE: this is important to ensure anyone reading from this stream does not loop
        // forever, it ensure EOF is reached on the reader side, by shutting down the writer
        tcp_stream.shutdown().await?;
        let mut response = response::Response::new();
        response.read(&mut tcp_stream).await?;
        println!("response that I got: {:#?}", response);
        Ok::<(), error::Error>(())
    }.await {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e.to_string());
            std::process::exit(1);
        }
    };
}
