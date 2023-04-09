/// Provides a trait definition that specifies the HCS protocol. This trait should be implemented
/// by a runtime.
#[cfg(feature = "protocol")]
pub mod protocol;

/// Provides a trait definition for `Data` that should be passed between the client and server.
/// Default `Data` structs are also defined here.
#[cfg(feature = "data")]
pub mod data;

/// Basic logger that adds color and a level filter.
#[cfg(feature = "logger")]
pub mod logger;

/// Read `.toml` configuration files. Adds a few type-specific parses.
#[cfg(feature = "config")]
pub mod config;

#[cfg(feature = "errors")]
pub mod errors;

/// Postgres database config + default tables + functions.
// TODO Add casts from database record to ChangeType
#[cfg(feature = "server_database")]
pub mod server_database;

/// Stores the server version, number of changes and local changes (used by [`client_detect_offline`] and [`client_detect_live`]).
#[cfg(feature = "client_database")]
pub mod client_database;

#[cfg(feature = "client_detect_offline")]
pub mod client_detect_offline;

#[cfg(feature = "client_detect_live")]
pub mod client_detect_live;
