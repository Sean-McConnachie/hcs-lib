use std::{fs, path};

use crate::client_database;

use super::storage_cases;

pub fn walk_storage(
    dir: &path::PathBuf,
    file_handler_config: &client_database::FileHandlerConfig,
    change_counter: &mut client_database::ChangeCounter,
) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path_buf = path.unwrap().path().to_path_buf();
        let file_paths =
            client_database::FilePaths::from_path_checked(path_buf.clone(), &file_handler_config)
                .unwrap();

        match file_paths.file_type() {
            client_database::Type::File => {
                // Case 7
                storage_cases::real_file_exists(&file_paths, change_counter);
            }
            client_database::Type::CustomMetadata => {
                // Case 8
                storage_cases::custom_metadata_exists(&file_paths, change_counter);
            }
            client_database::Type::Directory => {
                // Case 9
                storage_cases::directory_exists(&file_paths, change_counter);
                walk_storage(file_paths.path(), file_handler_config, change_counter);
            }
            client_database::Type::Symlink => {
                // Case 10
                storage_cases::symlink_exists();
            }
        }
    }
}
