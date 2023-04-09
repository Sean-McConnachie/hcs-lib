//! `File` default implementations:
//! - [`SymlinkCreate`](struct@SymlinkCreate) : `data_uid = 10`
//! - [`SymlinkDelete`](struct@SymlinkDelete) : `data_uid = 11`

mod create;
mod delete;

pub use create::SymlinkCreate;
pub use delete::SymlinkDelete;
