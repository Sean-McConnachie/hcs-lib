use super::Data;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SyncClientToServer {
    client_version: i32,
    number_of_changes: i32,
}

impl SyncClientToServer {
    pub fn new(client_version: i32, number_of_changes: i32) -> Self {
        Self {
            client_version,
            number_of_changes,
        }
    }

    pub fn client_version(&self) -> i32 {
        self.client_version
    }

    pub fn number_of_changes(&self) -> i32 {
        self.number_of_changes
    }
}

impl Data for SyncClientToServer {}
