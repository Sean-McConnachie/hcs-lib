//! `File` default implementations:
//! - [`FileCreate`](struct@FileCreate) : `data_uid = 5`
//! - [`FileModify`](struct@FileModify) : `data_uid = 6`
//! - [`FileMove`](struct@FileMove) : `data_uid = 7`
//! - [`FileDelete`](struct@FileDelete) : `data_uid = 8`
//! - [`FileUndoDelete`](struct@FileUndoDelete) : `data_uid = 9`

mod create;
mod delete;
mod modify;
mod r#move;
mod undo_delete;

pub use create::FileCreate;
pub use delete::FileDelete;
pub use modify::FileModify;
pub use r#move::FileMove;
pub use undo_delete::FileUndoDelete;
