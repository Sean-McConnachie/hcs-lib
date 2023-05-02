use sqlx::Row;

use crate::data::{ChangeEvent, Data, DirectoryEvent};

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct DirectoryCreate {
    path: String,
}

impl DirectoryCreate {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Data for DirectoryCreate {}

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for DirectoryCreate {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            path: row.get("path"),
        }
    }
}

impl Into<ChangeEvent> for DirectoryCreate {
    fn into(self) -> ChangeEvent {
        ChangeEvent::Directory(DirectoryEvent::Create(self))
    }
}
