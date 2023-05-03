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

    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Data for Greeting {}

impl<E, D> TryInto<Greeting> for super::Transmission<E, D> {
    type Error = ();

    fn try_into(self) -> Result<Greeting, <super::Transmission<E, D> as TryInto<Greeting>>::Error> {
        match self {
            super::Transmission::Greeting(greeting) => Ok(greeting),
            _ => Err(()),
        }
    }
}
