use super::super::Data;

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FileModify {
    size: usize,
    path: String,
}

impl FileModify {
    pub fn new(size: usize, path: String) -> Self {
        Self { size, path }
    }
}

impl Data for FileModify {}
