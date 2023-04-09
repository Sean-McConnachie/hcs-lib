use super::Data;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ServerVersion {
    server_version: i64,
}

impl ServerVersion {
    pub fn new(server_version: i64) -> Self {
        Self { server_version }
    }
}

impl Data for ServerVersion {}
