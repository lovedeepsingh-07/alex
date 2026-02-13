use crate::error;
use std::collections::HashMap;
use lofty::file::AudioFile;

#[derive(Debug, Default)]
pub struct Audio {
    pub slug: String,
    pub name_without_ext: String,
    pub extension: String,
    pub path: std::path::PathBuf,
    pub duration: std::time::Duration,
}

pub type AudioIndex = HashMap<String, Audio>;

pub fn index_audio_files(folder_path: &std::path::PathBuf) -> Result<AudioIndex, error::Error> {
    let mut index = HashMap::new();
    for entry in walkdir::WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry = entry.path();
        if entry.is_file() {
            let metadata = match extract_metadata(entry) {
                Ok(out) => out,
                Err(e) => {
                    if let error::Error::InvalidInputError(_) = e {
                        continue;
                    }
                    return Err(e);
                }
            };
            index.insert(metadata.slug.clone(), metadata);
        };
    }
    Ok(index)
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
            error::Error::FSError(
                format!("Failed to get the extension for a file entry with name, {}", file_name_with_ext),
            )
        })?
        .to_string_lossy()
        .to_string();

    // filetering
    if !["ogg"].contains(&extension.as_str()) {
        return Err(error::Error::InvalidInputError(String::new()));
    }

    // conversion into usable data
    let file_name_without_ext = match file_name_with_ext.strip_suffix(&format!(".{}", extension)) {
        Some(out) => out,
        None => return Err(error::Error::FSError(format!("Invalid file name, {}", file_name_with_ext))),
    };

    // audio metadata
    let tagged_file = lofty::read_from_path(entry_path)?;
    let duration = tagged_file.properties().duration();

    output.slug = sanitize_string(file_name_without_ext).to_lowercase();
    output.name_without_ext = file_name_without_ext.to_string();
    output.extension = extension;
    output.path = entry_path.to_path_buf();
    output.duration = duration;
    Ok(output)
}

fn sanitize_string(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        let filtered_char = match c {
            'a'..'z' | 'A'..'Z' | '0'..'9' => c,
            _ => '_'
        };
        output.push(filtered_char);
    }
    output
}
