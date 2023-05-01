use std::fs;

use crate::client_database;

pub fn delete_dir(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    {
        // Delete custom metadata if exists
        if file.custom_metadata_path().exists() {
            fs::remove_file(file.custom_metadata_path()).unwrap();
        }
    }
    {
        // Delete directory in symlink if exists
        if file.symlink_dir_path().exists() {
            fs::remove_dir(file.symlink_dir_path()).unwrap();
        }
    }
    {
        // Delete directory in storage if exists
        if file.storage_dir_path().exists() {
            fs::remove_dir(file.storage_dir_path()).unwrap();
        }
    }
    {
        // Add delete directory change
        let delete_change = format!("delete_dir\n{}", file.relative_path().display());
        fs::write(change_counter.next_path(), delete_change).unwrap();
    }
}
