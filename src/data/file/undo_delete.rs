use sqlx::Row;

use crate::data::{ChangeEvent, Data, FileEvent};

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FileUndoDelete {
    path: String,
}

impl FileUndoDelete {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl Data for FileUndoDelete {}

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for FileUndoDelete {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            path: row.get("path"),
        }
    }
}

impl Into<ChangeEvent> for FileUndoDelete {
    fn into(self) -> ChangeEvent {
        ChangeEvent::File(FileEvent::UndoDelete(self))
    }
}
