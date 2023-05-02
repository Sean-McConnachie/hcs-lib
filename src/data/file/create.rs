use super::super::Data;

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FileCreate {
    size: usize,
    path: String,
}

impl FileCreate {
    pub fn new(size: usize, path: String) -> Self {
        Self { size, path }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Data for FileCreate {}
