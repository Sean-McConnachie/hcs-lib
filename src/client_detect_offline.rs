#![allow(dead_code)]

//! This module detects file changes (i.e. Create, Move, Modify, Delete etc...).
//!
//! There are two directories:
//! - `symlink_directory` `A`
//! - `storage_directory` `B`
//!
//! ## Cases when iterating through `A` (symlink directory)Pass
//! - Symlink exists
//!     - Points to file in `A` **(1)**
//!     - Points to file in `B` **(2)**
//!     - Points to file outside of either `A` or `B` **(3)**
//! - Real file exists **(4)**
//! - Directory exists **(5)**
//! - Custom metadata file exists **(6)**
//!
//! ## Cases when iterating through `B` (storage directory)
//! - Real file exists **(7)**
//! - Custom metadata file exists **(8)**
//! - Directory exists **(9)**
//! - Symlink exists **(10)**
//!
//! ## Walk symlink dir
//! ### Case **`1`**
//! Ignore for now
//! In the future, keep a record of currently active symlinks. Send relevant symlinks to the server.
//!
//! ### Case **`2`**
//! 1) Check if the file that it points to exists:
//!
//! **Yes**
//!
//! i) Pass.
//!
//! **No**
//!
//! i) Delete symlink.
//!
//! ii) Delete custom metadata file if exists.
//!
//! iii) Add `delete` change.
//!
//! 2) Check if the relative path of the symlink pointer and the `points to` of the symlink are the same:
//!
//! **Yes**
//!
//! i) Pass.
//!
//! **No**
//!
//! i) Move the real file to path of symlink `point to` in `B` directory.
//!
//! ii) Move custom metadata to path of symlink `point to` in `B` directory.
//!
//! iii) Add `move` change.
//!
//! ### Case **`3`**
//! Ignore the file. Continue.
//!
//! ### Case **`4`**
//! 1) Move the file to the `B` directory.
//! 2) Create custom metadata file with the last modified time of the file.
//! 3) Create a symlink in the `A` directory that points to the file in the `B` directory.
//! 2) Add `create` change.
//!
//! ### Case **`5`**
//! 1) Check if directory and custom metadata file exists in `B` directory:
//!
//! **Yes**
//!
//! i) Pass. TODO: Do not iterate through that sub-directory.
//!
//! **No**
//!
//! i) Create directory in `B` directory.
//!
//! ii) Create custom metadata file in `B` directory.
//!
//! iii) Add `create` change.
//!
//! ### Case **`6`**
//! Delete the file.
//!
//! ## Walk real directory
//! ### Case **`7`**
//! 1) Check if custom metadata file exists `and` symlink in `A` exists:
//!
//! **Yes**
//!
//! Pass.
//!
//! **No**
//!
//! i) Delete custom metadata if exists.
//!
//! ii) Delete symlink if exists.
//!
//! iii) Add `delete` change.
//!
//! iv) Continue.
//!
//! 2) Check if the last modified of the real file and custom metadata file are the same:
//!
//! **Yes**
//!
//! i) Pass.
//!
//! **No**
//!
//! i) Add a `modified` change.
//!
//! ### Case **`8`**
//! 1) Check if real file exists `and` symlink in `A` exists:
//!
//! **Yes**
//!
//! Pass.
//!
//! **No**
//!
//! i) Delete symlink if exists.
//!
//! ii) Delete real file if exists.
//!
//! iii) Add `delete` change.
//!
//! ### Case **`9`**
//! 1) Check if custom metadata file exists `and` symlink in `A` exists:
//!
//! **Yes**
//!
//! Pass.
//!
//! **No**
//!
//! i) Delete custom metadata file if exists.
//!
//! ii) Delete symlink file if exists.
//!
//! iii) Add `delete` change.
//!
//! ### Case **`10`**
//! Ignore the file. Continue.
//!
use crate::client_database;

mod storage_cases;
mod symlink_cases;
mod walk_storage_dir;
mod walk_symlink_dir;

fn detect_offline_changes(file_handler_config: &client_database::FileHandlerConfig) {
    let mut change_counter =
        client_database::ChangeCounter::init(&file_handler_config.program_data_directory);

    client_database::make_blank_file(
        &file_handler_config.program_data_directory,
        change_counter.increment(),
    );

    walk_symlink_dir::walk_symlink(
        &file_handler_config.symlink_directory,
        &file_handler_config,
        &mut change_counter,
    );
    walk_storage_dir::walk_storage(
        &file_handler_config.storage_directory,
        &file_handler_config,
        &mut change_counter,
    );
}
