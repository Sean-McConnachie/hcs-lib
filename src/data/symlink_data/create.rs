use super::super::Data;

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SymlinkCreate {
    path: String,
    links_to: String,
}

impl SymlinkCreate {
    pub fn new(path: String, links_to: String) -> Self {
        Self { path, links_to }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn links_to(&self) -> &str {
        &self.links_to
    }
}

impl Data for SymlinkCreate {}
