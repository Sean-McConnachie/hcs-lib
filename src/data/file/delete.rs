use super::super::Data;

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FileDelete {
    path: String,
}

impl FileDelete {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

impl Data for FileDelete {}
