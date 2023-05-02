use super::super::Data;

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FileMove {
    from_path: String,
    to_path: String,
}

impl FileMove {
    pub fn new(from_path: String, to_path: String) -> Self {
        Self { from_path, to_path }
    }

    pub fn from_path(&self) -> &str {
        &self.from_path
    }

    pub fn to_path(&self) -> &str {
        &self.to_path
    }
}

impl Data for FileMove {}
