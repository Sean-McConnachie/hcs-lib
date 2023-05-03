use super::Data;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ServerVersion {
    server_version: i32,
}

impl ServerVersion {
    pub fn new(server_version: i32) -> Self {
        Self { server_version }
    }

    pub fn server_version(&self) -> i32 {
        self.server_version
    }
}

impl Data for ServerVersion {}
