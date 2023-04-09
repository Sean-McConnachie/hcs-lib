use super::super::Data;

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct DirectoryMove {
    from_path: String,
    to_path: String,
}

impl DirectoryMove {
    pub fn new(from_path: String, to_path: String) -> Self {
        Self { from_path, to_path }
    }
}

impl Data for DirectoryMove {}
