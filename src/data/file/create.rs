use sqlx::Row;

use crate::data::{ChangeEvent, Data, FileEvent};

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FileCreate {
    size: u64,
    path: String,
}

impl FileCreate {
    pub fn new(size: u64, path: String) -> Self {
        Self { size, path }
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }
}

impl Data for FileCreate {}

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for FileCreate {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            size: 0,
            path: row.get("path"),
        }
    }
}

impl Into<ChangeEvent> for FileCreate {
    fn into(self) -> ChangeEvent {
        ChangeEvent::File(FileEvent::Create(self))
    }
}
