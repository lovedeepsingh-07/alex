use crate::{constants, error};
use tokio::io::AsyncBufReadExt;

#[derive(Debug)]
pub enum Command {
    Status,
}

#[allow(dead_code)]
pub struct Player {
    output_stream: rodio::OutputStream,
    playing_sink: rodio::Sink,
    music_files: Vec<std::path::PathBuf>,
}
impl Player {
    pub fn new() -> Self {
        let home_dir = std::env::home_dir().unwrap();
        let music_folder_path = home_dir.join("Music");
        let mut music_files: Vec<std::path::PathBuf> = Vec::new();
        for entry in walkdir::WalkDir::new(music_folder_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry = entry.path();
            if entry.is_file() {
                music_files.push(entry.to_path_buf());
            };
        }
        let output_stream = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let playing_sink = rodio::Sink::connect_new(&output_stream.mixer());
        Player {
            music_files,
            output_stream,
            playing_sink,
        }
    }
}

#[derive(Debug)]
pub struct Daemon {
    pub sender: crossbeam::channel::Sender<Command>,
    pub receiver: crossbeam::channel::Receiver<Command>,
}
impl Daemon {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::bounded::<Command>(constants::COMMAND_CAP);

        Daemon {
            sender: tx,
            receiver: rx,
        }
    }
    pub async fn run(&self) -> Result<(), error::Error> {
        let handler_rx = self.receiver.clone();
        tokio::spawn(async move {
            let player = Player::new();
            loop {
                match handler_rx.try_recv() {
                    Ok(cmd) => {
                        println!("{:#?}", cmd);
                        let file = std::fs::File::open(&player.music_files[0]).unwrap();
                        let source = rodio::Decoder::try_from(file).unwrap();
                        player.playing_sink.stop();
                        player.playing_sink.append(source);
                        // player.output_stream.mixer().add(source);
                    }
                    Err(crossbeam::channel::TryRecvError::Empty) => {}
                    Err(crossbeam::channel::TryRecvError::Disconnected) => {
                        eprintln!("the channel is disconnected");
                    }
                }
            }
        });

        let listener = tokio::net::TcpListener::bind(constants::SERVER_ADDRESS).await?;
        loop {
            let (mut tcp_stream, _) = listener.accept().await?;
            let mut reader = tokio::io::BufReader::new(&mut tcp_stream);
            let mut line = String::new();
            reader.read_line(&mut line).await?;
            match line.as_str() {
                "STATUS" => {
                    self.sender.send(Command::Status).unwrap();
                }
                _ => {}
            }
        }
    }
}
