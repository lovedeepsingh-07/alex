use crate::error;
use std::collections::HashMap;

pub(crate) struct Audio {
    pub(crate) path: std::path::PathBuf,
}

pub(crate) type AudioIndex = HashMap<String, Audio>;

pub(crate) fn index_audio_files() -> Result<AudioIndex, error::Error> {
    let mut index = HashMap::new();
    let home_dir = std::env::home_dir().ok_or_else(|| {
        error::Error::FSError("Failed to get the home directory path".to_string())
    })?;
    let music_folder_path = home_dir.join("Music");
    for entry in walkdir::WalkDir::new(music_folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry = entry.path();
        if entry.is_file() {
            let file_name = entry
                .file_name()
                .ok_or_else(|| {
                    error::Error::FSError("Failed to get the filename for a file entry".to_string())
                })?
                .to_string_lossy()
                .to_string();
            let extension = entry
                .extension()
                .ok_or_else(|| {
                    error::Error::FSError(
                        "Failed to get the extension for a file entry".to_string(),
                    )
                })?
                .to_string_lossy()
                .to_string();
            if !["ogg"].contains(&extension.as_str()) {
                continue;
            }
            let label = match file_name.strip_suffix(&format!(".{}", extension)) {
                Some(out) => out,
                None => file_name.as_str(),
            };
            index.insert(
                label.to_string(),
                Audio {
                    path: entry.to_path_buf(),
                },
            );
        };
    }
    Ok(index)
}
