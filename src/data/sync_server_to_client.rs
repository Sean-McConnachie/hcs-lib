use super::Data;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SyncServerToClient {
    client_version: i64,
}

impl SyncServerToClient {
    pub fn new(client_version: i64) -> Self {
        Self { client_version }
    }
}

impl Data for SyncServerToClient {}
