use std::fs;

use log::debug;

use crate::client_database;

/// This function assumes that the real file is currently located in the storage directory.
pub fn create_file(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    debug!("`create_file`: `{}`", file.relative_path().display());
    {
        // Create a custom metadata file
        let last_modified =
            client_database::CustomMetadata::last_modified_of_file(&file.storage_dir_path())
                .unwrap();
        let custom_metadata_file = client_database::CustomMetadata::new(last_modified);
        custom_metadata_file.write_to_file(&file).unwrap();
    }
    {
        // Create a symlink in the symlink directory
        symlink::symlink_file(&file.storage_dir_path(), &file.symlink_dir_path()).unwrap();
    }
    {
        // Add a `create file` change
        let create_change = format!("create_file\n{}", file.relative_path().to_str().unwrap());
        fs::write(change_counter.next_path(), create_change).unwrap();
    }
}
