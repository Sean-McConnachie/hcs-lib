use sqlx::Row;

use crate::data::{ChangeEvent, Data, FileEvent};

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FileModify {
    size: u64,
    path: String,
}

impl FileModify {
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

impl Data for FileModify {}

#[cfg(feature = "server")]
impl From<sqlx::postgres::PgRow> for FileModify {
    fn from(row: sqlx::postgres::PgRow) -> Self {
        Self {
            size: 0,
            path: row.get("path"),
        }
    }
}

impl Into<ChangeEvent> for FileModify {
    fn into(self) -> ChangeEvent {
        ChangeEvent::File(FileEvent::Modify(self))
    }
}
