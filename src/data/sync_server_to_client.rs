use super::Data;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SyncServerToClient {
    client_version: i32,
}

impl SyncServerToClient {
    pub fn new(client_version: i32) -> Self {
        Self { client_version }
    }

    pub fn client_version(&self) -> i32 {
        self.client_version
    }
}

impl Data for SyncServerToClient {}
