use std::fs;

use log::debug;

use crate::client_database;

pub fn delete_file(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    debug!("`delete_file`: `{}`", file.relative_path().display());
    // Delete symlink
    {
        let symlink_path = file.symlink_dir_path();
        if fs::read_link(&symlink_path).is_ok() {
            symlink::remove_symlink_file(symlink_path).unwrap();
        }
    }
    {
        // Delete custom metadata file if exists
        let custom_metadata_file = file.custom_metadata_path();
        if custom_metadata_file.exists() {
            fs::remove_file(custom_metadata_file).unwrap();
        }
    }
    {
        // Delete real file if exists
        let real_file = file.storage_dir_path();
        if real_file.exists() {
            fs::remove_file(real_file).unwrap();
        }
    }
    {
        // Add file delete change
        let delete_change = format!("delete_file\n{}", file.relative_path().to_str().unwrap());
        fs::write(change_counter.next_path(), delete_change).unwrap();
    }
}
