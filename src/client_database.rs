mod blank_file;
mod change_counter;
mod custom_metadata;
mod file_types;
mod server_version;

pub use blank_file::make_blank_file;
pub use change_counter::ChangeCounter;
pub use custom_metadata::CustomMetadata;
pub use file_types::{FilePaths, FileType};
pub use server_version::ServerVersion;

use crate::config::parse_path_buf;

use std::path;

#[derive(serde::Deserialize, Debug, Clone, Default, PartialEq)]
pub struct FileHandlerConfig {
    /// `storage_directory` is the directory where the real files are stored.
    #[serde(deserialize_with = "parse_path_buf")]
    pub storage_directory: path::PathBuf,

    /// `symlink_directory` is the directory where the symlinks are stored.
    /// This is the directory that the user would normally interact with.
    #[serde(deserialize_with = "parse_path_buf")]
    pub symlink_directory: path::PathBuf,

    /// `temporary_directory` is the directory where temporary files are stored.
    /// This is where files that are being sent/received are stored before being
    /// moved into their respective directories.
    #[serde(deserialize_with = "parse_path_buf")]
    pub temporary_directory: path::PathBuf,

    /// `program_data_directory` is the directory where the program data is stored.
    /// This includes:
    /// - `server_version`
    /// - `change_count`
    /// - `un-synced changes`
    #[serde(deserialize_with = "parse_path_buf")]
    pub program_data_directory: path::PathBuf,
}
