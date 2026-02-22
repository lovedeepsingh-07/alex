// NOTE: Player has two components, "Engine" and "Storage", the engine is what actually plays
// a song, the storage as it's name suggests, keeps the songs in memory with the index for
// searching and speeding up the lookup process, the "Player" is what combines both of these
// components with a queue implementation to actually run everything

pub mod engine;
pub use engine::Engine;

pub mod storage;
pub use storage::Audio;
pub use storage::AudioID;
pub use storage::Storage;

use crate::error;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Player {
    root_folder_path: std::path::PathBuf,
    engine: Engine,
    pub storage: Storage,
    pub queue: VecDeque<String>,
    current_audio: Option<AudioID>,
}

impl Player {
    pub fn new(root_folder_path: std::path::PathBuf) -> Result<Self, error::Error> {
        let engine = Engine::new()?;
        let storage = Storage::generate(&root_folder_path)?;
        let queue: VecDeque<String> = VecDeque::new();

        Ok(Self {
            root_folder_path,
            engine,
            storage,
            queue,
            current_audio: None,
        })
    }
    pub fn get_current_audio(&self) -> &Option<AudioID> {
        &self.current_audio
    }
    pub fn is_paused(&self) -> bool {
        self.engine.is_paused()
    }
    pub fn is_queue_empty(&self) -> bool {
        self.queue.is_empty()
    }
    // NOTE: the "on_next" function allows users to run some arbitrary code whenever the current
    // audio ends and the next audio from the queue begins
    pub fn tick<F>(&mut self, on_next: F) -> Result<(), error::Error>
    where
        F: FnOnce(&str) -> Result<(), error::Error>,
    {
        match (self.engine.is_sink_empty(), self.queue.is_empty()) {
            (true, true) => self.current_audio = None,
            (true, false) => {
                let next_audio_id = self.queue.pop_front().ok_or_else(|| {
                    error::Error::PlayerError(
                        "Failed to get the next song from the playing queue".to_string(),
                    )
                })?;
                let audio = self.storage.get_audio(&next_audio_id)?.clone();
                self.play(&audio)?;
                (on_next)(audio.get_title())?;
            }
            (false, true) => {
                // NOTE: Whenever there is an audio playing but no more audios are left in the queue,
                // that means that we have chance to repopulate the queue in order to keep the
                // playback running forever
                self.populate_queue()?;
            }
            (false, false) => {}
        }
        Ok(())
    }
    pub fn play(&mut self, audio: &Audio) -> Result<(), error::Error> {
        let id = audio.get_id().clone();
        self.engine.play(audio)?;
        self.current_audio = Some(id);
        Ok(())
    }
    pub fn pause(&mut self) {
        self.engine.pause();
    }
    pub fn resume(&mut self) {
        self.engine.resume();
    }
    pub fn next(&self) -> Result<(), error::Error> {
        Ok(())
    }
    pub fn clear_sink(&mut self) {
        self.engine.clear_sink();
    }
    pub fn clear_queue(&mut self) {
        self.clear_sink();
        self.queue.clear();
    }
    pub fn reload_storage(&mut self) -> Result<(), error::Error> {
        self.storage = Storage::generate(&self.root_folder_path)?;
        Ok(())
    }
    pub fn populate_queue(&mut self) -> Result<(), error::Error> {
        let mut ids: Vec<String> = self
            .storage
            .get_audio_map()
            .iter()
            .filter_map(|(id, _)| {
                if let Some(ref current_audio_id) = self.current_audio {
                    if id == current_audio_id {
                        return None;
                    }
                    return Some(id.to_string());
                }
                return Some(id.to_string());
            })
            .collect();
        ids.shuffle(&mut rand::rng());
        self.queue = ids.into();
        Ok(())
    }
}
