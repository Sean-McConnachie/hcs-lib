use sqlx::Row;

use crate::data::{ChangeEvent, Data, DirectoryEvent};

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct DirectoryDelete {
    path: String,
}

impl DirectoryDelete {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Data for DirectoryDelete {}

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for DirectoryDelete {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            path: row.get("path"),
        }
    }
}

impl Into<ChangeEvent> for DirectoryDelete {
    fn into(self) -> ChangeEvent {
        ChangeEvent::Directory(DirectoryEvent::Delete(self))
    }
}
