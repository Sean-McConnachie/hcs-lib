use sqlx::Row;

use crate::data::{ChangeEvent, Data, DirectoryEvent};

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

    pub fn from_path(&self) -> &str {
        &self.from_path
    }

    pub fn to_path(&self) -> &str {
        &self.to_path
    }
}

impl Data for DirectoryMove {}

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for DirectoryMove {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            from_path: row.get("from_path"),
            to_path: row.get("to_path"),
        }
    }
}

impl Into<ChangeEvent> for DirectoryMove {
    fn into(self) -> ChangeEvent {
        ChangeEvent::Directory(DirectoryEvent::Move(self))
    }
}
