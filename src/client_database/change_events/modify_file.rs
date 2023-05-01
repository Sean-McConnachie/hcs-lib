use std::fs;

use crate::client_database;

pub fn modify_file(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    let mut custom_metadata = client_database::CustomMetadata::read_from_file(file).unwrap();
    let modified_change = format!("file_modified\n{}", file.relative_path().to_str().unwrap(),);
    fs::write(change_counter.next_path(), modified_change).unwrap();

    let real_last_modified =
        client_database::CustomMetadata::last_modified_of_file(file.storage_dir_path()).unwrap();
    custom_metadata.set_last_modified(real_last_modified);
    custom_metadata.write_to_file(file).unwrap();
}
