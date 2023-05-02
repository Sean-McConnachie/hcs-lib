use sqlx::Row;

use crate::data::{ChangeEvent, Data, FileEvent};

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

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for FileMove {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            from_path: row.get("from_path"),
            to_path: row.get("to_path"),
        }
    }
}

impl Into<ChangeEvent> for FileMove {
    fn into(self) -> ChangeEvent {
        ChangeEvent::File(FileEvent::Move(self))
    }
}
