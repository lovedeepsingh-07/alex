use crate::{error, utils};
use lofty::file::AudioFile;
use std::collections::{HashMap, HashSet};

pub type AudioID = String;

#[derive(Debug, Default)]
pub struct Audio {
    id: AudioID,
    name_without_ext: String,
    extension: String,
    path: std::path::PathBuf,
    duration: std::time::Duration,
}
impl Audio {
    pub fn searchable_fields(&self) -> Vec<(&AudioID, f64)> {
        vec![(&self.id, 1.0)]
    }
    pub fn get_duration(&self) -> std::time::Duration {
        self.duration
    }
    pub fn get_id(&self) -> &AudioID {
        return &self.id
    }
    pub fn get_extension(&self) -> &str {
        return &self.extension
    }
}

#[derive(Debug, Default)]
pub struct Storage {
    audios: HashMap<AudioID, Audio>,
    index: HashMap<String, HashSet<AudioID>>,
}
impl Storage {
    pub fn audios(&self) -> &HashMap<AudioID, Audio> {
        return &self.audios;
    }
    pub fn get_search_candidates(&self, query_tokens: &[String]) -> HashSet<AudioID> {
        let mut candidates: HashSet<AudioID> = HashSet::new();
        for query_tok in query_tokens {
            for (index_tok, audio_ids) in self.index.iter() {
                if index_tok.starts_with(query_tok.as_str()) {
                    candidates.extend(audio_ids.clone());
                    continue;
                }
                if query_tok.len() >= 3 {
                    let dist = strsim::levenshtein(query_tok, index_tok);
                    let threshold = match query_tok.len() {
                        3..=4 => 1,
                        5..=7 => 2,
                        _ => 3,
                    };
                    if dist < threshold {
                        candidates.extend(audio_ids.clone());
                    }
                }
            }
        }
        return candidates;
    }
    pub fn generate(root_folder_path: &std::path::PathBuf) -> Result<Self, error::Error> {
        let mut storage = Storage::default();
        for entry in walkdir::WalkDir::new(root_folder_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry = entry.path();
            if entry.is_file() {
                let metadata = match Self::extract_audio_file_metadata(entry) {
                    Ok(out) => out,
                    Err(e) => {
                        // NOTE: checkout the "extract_audio_file_metadata" method for why this error is
                        // specifically ruled out
                        if let error::Error::InvalidInputError(_) = e {
                            continue;
                        }
                        return Err(e);
                    }
                };

                // actual token based indexing
                let id_tokens =
                    utils::remove_stop_words(utils::tokenize_string(metadata.id.as_str()));
                for token in id_tokens.iter() {
                    match storage.index.get_mut(token.as_str()) {
                        Some(id_set) => {
                            id_set.insert(metadata.id.clone());
                        }
                        None => {
                            let mut id_set: HashSet<AudioID> = HashSet::new();
                            id_set.insert(metadata.id.clone());
                            storage.index.insert(token.clone(), id_set);
                        }
                    }
                }

                // NOTE: this is done at the end because some things are needed to be done to the
                // metadata before moving it anywhere
                storage.audios.insert(metadata.id.clone(), metadata);
            };
        }
        Ok(storage)
    }
    fn extract_audio_file_metadata(entry_path: &std::path::Path) -> Result<Audio, error::Error> {
        let mut output = Audio::default();

        // file system related data
        let file_name_with_ext = entry_path
            .file_name()
            .ok_or_else(|| {
                error::Error::FSError("Failed to get the filename for a file entry".to_string())
            })?
            .to_string_lossy();
        let extension = entry_path
            .extension()
            .ok_or_else(|| {
                error::Error::FSError(format!(
                    "Failed to get the extension for a file entry with name, {}",
                    file_name_with_ext
                ))
            })?
            .to_string_lossy()
            .to_string();

        // filtering
        if !["ogg"].contains(&extension.as_str()) {
            // NOTE: we have to signal to the loop running outside of this function that we have to
            // skip a certain item from the loop(basically "continue" the loop), we specifically use
            // the "InvalidInputError" for that
            return Err(error::Error::InvalidInputError(String::new()));
        }

        // conversion into usable data
        let file_name_without_ext = match file_name_with_ext.strip_suffix(&format!(".{}", extension)) {
            Some(out) => out,
            None => {
                return Err(error::Error::FSError(format!(
                    "Invalid file name, {}",
                    file_name_with_ext
                )));
            }
        };

        // audio metadata
        let tagged_file = lofty::read_from_path(entry_path)?;
        let duration = tagged_file.properties().duration();

        output.id = utils::sanitize_string(file_name_without_ext).to_lowercase();
        output.name_without_ext = file_name_without_ext.to_string();
        output.extension = extension;
        output.path = entry_path.to_path_buf();
        output.duration = duration;
        Ok(output)
    }
}
