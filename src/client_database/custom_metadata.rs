use super::{FilePaths, FileType};

use std::{fs, path, time};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CustomMetadata {
    last_modified: u64,
}

impl CustomMetadata {
    pub fn new(last_modified: u64) -> Self {
        Self { last_modified }
    }

    pub fn set_last_modified(&mut self, new_last_modified: u64) {
        self.last_modified = new_last_modified
    }

    pub fn last_modified(&self) -> u64 {
        self.last_modified
    }
}

// impl TryFrom<FilePaths<T>> for CustomMetadata {
//     type Error = std::io::Error;

//     fn try_from(file_paths: FilePaths) -> Result<Self, Self::Error> {
//         Self::read_from_file(&file_paths)
//     }
// }

impl CustomMetadata {
    pub fn write_to_file(&self, file_paths: &FilePaths) -> Result<(), std::io::Error> {
        let custom_metadata_path = file_paths.custom_metadata_path();
        let contents = serde_json::to_string(&self)?;
        fs::write(custom_metadata_path, contents)?;
        Ok(())
    }

    pub fn read_from_file(file_paths: &FilePaths) -> Result<Self, std::io::Error> {
        let custom_metadata_path = file_paths.custom_metadata_path();
        let bytes = fs::read(custom_metadata_path)?;
        let custom_metadata = serde_json::from_slice(&bytes)?;
        Ok(custom_metadata)
    }

    pub fn last_modified_of_file(fp: &path::PathBuf) -> Result<u64, std::io::Error> {
        let modified = fp
            .metadata()?
            .modified()?
            .duration_since(time::UNIX_EPOCH)
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Could not get duration since UNIX_EPOCH",
                )
            })?
            .as_secs();
        Ok(modified)
    }
}
