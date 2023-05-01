use std::{env, fs};

use crate::client_database;

pub fn rm_dirs_ce_dirs_get_default_helpers() -> (
    client_database::FileHandlerConfig,
    client_database::ChangeCounter,
) {
    let current_working_directory = env::current_dir().unwrap();

    let file_handler_config = client_database::FileHandlerConfig::new(
        current_working_directory
            .join("_storage_dir".to_string())
            .to_str()
            .unwrap()
            .to_string(),
        current_working_directory
            .join("_symlink_dir".to_string())
            .to_str()
            .unwrap()
            .to_string(),
        current_working_directory
            .join("_temporary_dir".to_string())
            .to_str()
            .unwrap()
            .to_string(),
    );

    // remove all directories and create them again
    fs::remove_dir_all(&file_handler_config.storage_directory);
    fs::remove_dir_all(&file_handler_config.symlink_directory);
    fs::remove_dir_all(&file_handler_config.temporary_directory);
    fs::remove_dir_all(&file_handler_config.program_data_directory);
    fs::create_dir_all(&file_handler_config.storage_directory);
    fs::create_dir_all(&file_handler_config.symlink_directory);
    fs::create_dir_all(&file_handler_config.temporary_directory);
    fs::create_dir_all(&file_handler_config.program_data_directory);

    let mut change_counter =
        client_database::ChangeCounter::init(&file_handler_config.program_data_directory);
    (file_handler_config, change_counter)
}
