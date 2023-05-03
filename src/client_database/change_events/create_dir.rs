use std::fs;

use log::debug;

use crate::client_database::{self, custom_metadata};

pub fn create_dir(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    debug!("`create_dir`: `{}`", file.relative_path().display());
    {
        // Create directory in storage directory
        fs::create_dir(file.storage_dir_path()).unwrap();
    }
    {
        // Create custom metadata file in storage directory
        let last_modified =
            custom_metadata::CustomMetadata::last_modified_of_file(file.storage_dir_path())
                .unwrap();
        let custom_metadata_file = custom_metadata::CustomMetadata::new(last_modified);
        custom_metadata_file.write_to_file(file).unwrap();
    }
    {
        // Make `create dir` change event
        dbg!(file.storage_dir_path());
        let create_change = format!("create_dir\n{}", file.relative_path().to_str().unwrap());
        fs::write(change_counter.next_path(), create_change).unwrap();
    }
}
