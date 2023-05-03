use std::fs;

use log::debug;

use crate::client_database;

pub fn modify_file(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    debug!("`modify_file`: `{}`", file.relative_path().display());
    let mut custom_metadata = client_database::CustomMetadata::read_from_file(file).unwrap();
    let modified_change = format!("modify_file\n{}", file.relative_path().to_str().unwrap(),);
    fs::write(change_counter.next_path(), modified_change).unwrap();

    let real_last_modified =
        client_database::CustomMetadata::last_modified_of_file(file.storage_dir_path()).unwrap();
    custom_metadata.set_last_modified(real_last_modified);
    custom_metadata.write_to_file(file).unwrap();
}
