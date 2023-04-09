use super::Data;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SyncClientToServer {
    client_version: i64,
    number_of_changes: i64,
}

impl SyncClientToServer {
    pub fn new(client_version: i64, number_of_changes: i64) -> Self {
        Self {
            client_version,
            number_of_changes,
        }
    }
}

impl Data for SyncClientToServer {}
