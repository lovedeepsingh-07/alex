use crate::{error, utils};
use lofty::file::AudioFile;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct Audio {
    pub slug: String,
    pub name_without_ext: String,
    pub extension: String,
    pub path: std::path::PathBuf,
    pub duration: std::time::Duration,
}

#[derive(Debug, Default)]
pub struct Storage {
    pub audios: HashMap<String, Audio>,
    pub index: HashMap<String, HashSet<String>>,
}

// NOTE: the name of this function is a little misleading, this function handles a lot of things
// along with indexing the audio files, basically, if anything is required to be stored for later
// use, this function stores that thing, if you can understand that, good
pub fn index_audio_files(folder_path: &std::path::PathBuf) -> Result<Storage, error::Error> {
    let mut storage = Storage::default();
    for entry in walkdir::WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry = entry.path();
        if entry.is_file() {
            let metadata = match extract_metadata(entry) {
                Ok(out) => out,
                Err(e) => {
                    // NOTE: checkout the "extract_metadata" function for why this error is
                    // specifically ruled out
                    if let error::Error::InvalidInputError(_) = e {
                        continue;
                    }
                    return Err(e);
                }
            };

            // actual token based indexing
            let slug_tokens =
                utils::remove_stop_words(utils::tokenize_string(metadata.slug.as_str()));
            for token in slug_tokens.iter() {
                match storage.index.get_mut(token.as_str()) {
                    Some(slug_set) => {
                        slug_set.insert(metadata.slug.clone());
                    }
                    None => {
                        let mut slug_set = HashSet::new();
                        slug_set.insert(metadata.slug.clone());
                        storage.index.insert(token.clone(), slug_set);
                    }
                }
            }

            // NOTE: this is done at the end because some things are needed to be done to the
            // metadata before moving it anywhere
            storage.audios.insert(metadata.slug.clone(), metadata);
        };
    }
    Ok(storage)
}

fn extract_metadata(entry_path: &std::path::Path) -> Result<Audio, error::Error> {
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

    output.slug = utils::sanitize_string(file_name_without_ext).to_lowercase();
    output.name_without_ext = file_name_without_ext.to_string();
    output.extension = extension;
    output.path = entry_path.to_path_buf();
    output.duration = duration;
    Ok(output)
}
