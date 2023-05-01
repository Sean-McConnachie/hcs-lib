use log::debug;

use crate::client_database;

pub fn modify_dir(file: &client_database::FilePaths) {
    debug!("`modify_dir`: `{}`", file.relative_path().display());
    {
        // Update custom metadata
        let directory_last_modified =
            client_database::CustomMetadata::last_modified_of_file(&file.symlink_dir_path())
                .unwrap();
        let mut custom_metadata = client_database::CustomMetadata::read_from_file(&file).unwrap();
        custom_metadata.set_last_modified(directory_last_modified);
        custom_metadata.write_to_file(&file).unwrap();
    }
}
