use super::Data;

/// Entry point for both the client and server. Used to determine whether the client and server are out of sync.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Greeting {
    version: String,
}

impl Greeting {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

impl Data for Greeting {}
