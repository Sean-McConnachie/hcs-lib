//!
//! Default `Data` implementations:
//! - [`Greeting`](struct@Greeting) : `data_uid = 0`
//! - [`Error`](struct@Error) : `data_uid = 1`
//! - [`SyncClientToServer`](struct@SyncClientToServer) : `data_uid = 2`
//! - [`SyncServerToClient`](struct@SyncServerToClient) : `data_uid = 3`
//! - [`ServerVersion`](struct@ServerVersion) : `data_uid = 4`
//! - [`file`] : `5 <= data_uid <= 9
//! - [`symlink`] : `10 <= data_uid <= 11`
//! - ['directory'] : `12 <= data_uid <= 15`

/// The `Data` trait is used to specify handle data types sent between the client and server.
/// Using serde traits means that a different datatype can be used. For example, the TCP runtime
/// uses bincode to serialize/deserialize structs into bytes. However, if the http requests or a
/// websocket were to be used, bytes cannot be sent, so instead, it can be serialized into a json.
pub trait Data: serde::Serialize + serde::Deserialize<'static> {}

mod directory;
mod error;
mod file;
mod greeting;
mod server_version;
mod symlink_data;
mod sync_client_to_server;
mod sync_server_to_client;

pub use directory::*;
pub use error::{Error, ErrorType};
pub use file::*;
pub use greeting::Greeting;
pub use server_version::ServerVersion;
pub use symlink_data::*;
pub use sync_client_to_server::SyncClientToServer;
pub use sync_server_to_client::SyncServerToClient;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum Transmission<E, D> {
    Greeting(Greeting),
    Error(Error<E>),
    SyncClientToServer(SyncClientToServer),
    SyncServerToClient(SyncServerToClient),
    ServerVersion(ServerVersion),
    EndConnection,
    SkipCurrent,
    ChangeEvent(ChangeEvent),
    Other(D),
}

impl<E, D> Data for Transmission<E, D>
where
    E: Data,
    D: Data,
{
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ChangeEvent {
    File(FileEvent),
    Directory(DirectoryEvent),
    Symlink(SymlinkEvent),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum FileEvent {
    Create(FileCreate),
    Modify(FileModify),
    Move(FileMove),
    Delete(FileDelete),
    UndoDelete(FileUndoDelete),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum DirectoryEvent {
    Create(DirectoryCreate),
    Move(DirectoryMove),
    Delete(DirectoryDelete),
    UndoDelete(DirectoryUndoDelete),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SymlinkEvent {
    Create(SymlinkCreate),
    Delete(SymlinkDelete),
}

#[cfg(test)]
mod tests {
    use crate::data::Transmission;

    use super::Data;
    use serde::{Deserialize, Serialize};

    #[test]
    fn data_trait() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Foo {
            val: String,
        }

        impl Data for Foo {}

        let val = "bar".to_string();
        let foo = Foo { val: val.clone() };

        let foo_json = serde_json::to_string(&foo).unwrap();

        let foo2: Foo = serde_json::from_str(&foo_json).unwrap();

        assert_eq!(foo, foo2);
    }

    #[test]
    fn default_types() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct Foo {
            val: String,
        }

        impl Data for Foo {}

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Plugins {
            Variant(Foo),
        }

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        enum Errors {
            Variant(Foo),
        }

        impl Data for Plugins {}

        let data_type = Transmission::<Errors, Plugins>::Other(Plugins::Variant(Foo {
            val: "bar".to_string(),
        }));

        let foo_json = serde_json::to_string(&data_type).unwrap();

        let foo2: Transmission<_, Plugins> = serde_json::from_str(&foo_json).unwrap();

        assert_eq!(data_type, foo2);
    }
}
