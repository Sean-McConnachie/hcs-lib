//! This module detects file changes (i.e. Create, Move, Modify, Delete etc...).
//!
//! There are two directories:
//! - `symlink_directory` `A`
//! - `storage_directory` `B`
//!
//! ## Cases when iterating through `A` (symlink directory)
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
//! ### Case **`1`**
//! 1)
//!
//! ### Case **`2`**
//!
//! ### Case **`3`**
//! Ignore the file. Continue.
//!
//! ### Case **`4`**
//!
//! ### Case **`5`**
//!
//! ### Case **`6`**
//! Delete the file.
//!
//! ### Case **`7`**
//!
//! ### Case **`8`**
//!
//! ### Case **`9`**
//!
//! ### Case **`10`**
//! Ignore the file. Continue.
//!
