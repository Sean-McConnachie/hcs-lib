use sqlx::Row;

use crate::data::{ChangeEvent, Data, SymlinkEvent};

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

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for SymlinkCreate {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            path: row.get("path"),
            links_to: row.get("links_to"),
        }
    }
}

impl Into<ChangeEvent> for SymlinkCreate {
    fn into(self) -> ChangeEvent {
        ChangeEvent::Symlink(SymlinkEvent::Create(self))
    }
}
