use sqlx::Row;

use crate::data::{ChangeEvent, Data, SymlinkEvent};

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SymlinkDelete {
    path: String,
}

impl SymlinkDelete {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Data for SymlinkDelete {}

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for SymlinkDelete {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            path: row.get("path"),
        }
    }
}

impl Into<ChangeEvent> for SymlinkDelete {
    fn into(self) -> ChangeEvent {
        ChangeEvent::Symlink(SymlinkEvent::Delete(self))
    }
}
