#![allow(unused_imports, dead_code)]
pub mod storage;
pub use storage::Storage;
pub use storage::Audio;
pub use storage::AudioID;

use crate::{error, protocol};
use std::collections::VecDeque;
// use rand::seq::SliceRandom;
// use colored::Colorize;

pub struct Player {
    root_folder_path: std::path::PathBuf,
    output_stream: rodio::OutputStream,
    sink: rodio::Sink,
    pub storage: Storage,
    pub queue: VecDeque<String>,
    current_audio: Option<AudioID>,
}
impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player")
         .field("root_folder_path", &self.root_folder_path)
         .field("output_stream", &"OutputStream")
         .field("sink", &"Sink")
         .field("storage", &self.storage)
         .finish()
    }
}

impl Player {
    pub fn new(root_folder_path: std::path::PathBuf) -> Result<Self, error::Error> {
        let mut output_stream = rodio::OutputStreamBuilder::open_default_stream()?;
        output_stream.log_on_drop(false);
        let sink = rodio::Sink::connect_new(&output_stream.mixer());
        let storage = Storage::generate(&root_folder_path)?;
        let queue: VecDeque<String> = VecDeque::new();

        Ok(Self {
            root_folder_path,
            output_stream,
            sink,
            storage,
            queue,
            current_audio: None
        })
    }

    pub fn get_current_audio(&self) -> &Option<AudioID> {
        &self.current_audio
    }
    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
    pub fn is_queue_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn tick(&self) -> Result<(), error::Error> {
        Ok(())
    }
    pub fn play(&self, input: &protocol::AudioInput) -> Result<(), error::Error> {
        let _ = input;
        Ok(())
    }
    pub fn push(&self, input: &protocol::AudioInput, next: bool) -> Result<(), error::Error> {
        let _ = input;
        let _ = next;
        Ok(())
    }
    pub fn pause(&mut self) -> Result<(), error::Error> {
        self.sink.pause();
        Ok(())
    }
    pub fn resume(&mut self) -> Result<(), error::Error> {
        self.sink.play();
        Ok(())
    }
    pub fn next(&self) -> Result<(), error::Error> {
        Ok(())
    }
    pub fn clear_queue(&mut self) -> Result<(), error::Error> {
        self.queue.clear();
        self.sink.clear();
        Ok(())
    }
    pub fn reload_storage(&mut self) -> Result<(), error::Error> {
        self.storage = Storage::generate(&self.root_folder_path)?;
        Ok(())
    }
}
