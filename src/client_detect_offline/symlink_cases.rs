use std::{fs, path};

use crate::client_database;

/// Case 1
pub fn symlink_points_to_symlink_dir(
    _file: &client_database::FilePaths,
    _file_handler_config: &client_database::FileHandlerConfig,
) -> () {
    // TODO: Ignore for now In the future, keep a record of currently active symlinks. Send relevant symlinks to the server.
    return;
}

/// Case 2
pub fn symlink_points_to_storage_dir(
    file: &client_database::FilePaths,
    file_handler_config: &client_database::FileHandlerConfig,
    change_counter: &mut client_database::ChangeCounter,
) {
    let points_to = file.points_to().unwrap();
    if points_to.exists() {
        // Check if relative path of point_to and actual relative path of the symlink are the same
        let points_to_relative_path = points_to
            .strip_prefix(&file_handler_config.storage_directory)
            .unwrap()
            .to_path_buf();
        let symlink_relative_path = file.relative_path();
        if &points_to_relative_path != symlink_relative_path {
            client_database::change_events::move_file(
                &points_to_relative_path,
                &symlink_relative_path,
                file_handler_config,
                change_counter,
                client_database::Type::Symlink,
            );
        }
    } else {
        client_database::change_events::delete_file(file, change_counter);
    }
}

/// Case 3
pub fn symlink_points_to_other_dir() -> () {
    return;
}

/// Case 4
pub fn real_file_exists(
    file: &client_database::FilePaths,
    file_handler_config: &client_database::FileHandlerConfig,
    change_counter: &mut client_database::ChangeCounter,
) {
    // First we need to move the file to the storage directory and rename to a unique
    // name if a file with the same name already exists there.
    // This should result in <file_name> (k) where k is the smallest
    // positive integer that results in a unique name.

    let unique_storage_path = {
        let storage_path =
            client_database::relative_to_real(&file.relative_path(), file_handler_config);

        if !storage_path.exists() {
            storage_path
        } else {
            let mut k = 1;
            let mut unique_storage_path = storage_path.clone();
            // check if the storage_path exists
            while unique_storage_path.exists() {
                let file_name = storage_path.file_stem().unwrap().to_str().unwrap();
                let file_ext = storage_path.extension().unwrap().to_str().unwrap();
                let new_file_name = format!("{} ({}).{}", file_name, k, file_ext);
                k += 1;

                unique_storage_path.set_file_name(new_file_name);
            }
            unique_storage_path
        }
    };

    // Move the file to the storage directory
    fs::rename(file.symlink_dir_path(), &unique_storage_path).unwrap();

    let unique_file =
        client_database::FilePaths::from_path_checked(unique_storage_path, file_handler_config)
            .unwrap();
    client_database::change_events::create_file(&unique_file, change_counter)
}

/// Case 5
pub fn directory_exists(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    // Check if directory and custom metadata exists in the storage directory
    if !(file.storage_dir_path().exists() && file.custom_metadata_path().exists()) {
        client_database::change_events::create_dir(file, change_counter);
    }
}

/// Case 6
pub fn custom_metadata_exists(custom_metadata_in_symlink_path: &path::PathBuf) {
    if custom_metadata_in_symlink_path.exists() {
        fs::remove_file(custom_metadata_in_symlink_path).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client_database, testing_utils::rm_dirs_ce_dirs_get_default_helpers};
    use std::{fs, path};

    #[test]
    fn test_custom_metadata_exists() {
        let (file_handler_config, mut _change_counter) = rm_dirs_ce_dirs_get_default_helpers();

        // create a file in the symlink directory
        let custom_metadata_path = &file_handler_config.symlink_directory.join(".file.txt.sc");

        fs::write(&custom_metadata_path, "Hello World!").unwrap();

        // call the function we want to test
        custom_metadata_exists(&custom_metadata_path);

        assert!(!custom_metadata_path.exists());
    }

    #[test]
    fn test_directory_exists() {
        let (file_handler_config, mut change_counter) = rm_dirs_ce_dirs_get_default_helpers();

        // create a directory in the symlink directory
        let real_dir_in_symlink = &file_handler_config.symlink_directory.join("dir");
        fs::create_dir(&real_dir_in_symlink).unwrap();

        // create a file in that directory
        fs::write(real_dir_in_symlink.join("file.txt"), "Hello World!").unwrap();

        // call the function we want to test
        let file = client_database::FilePaths::from_path_checked(
            real_dir_in_symlink.clone(),
            &file_handler_config,
        )
        .unwrap();
        directory_exists(&file, &mut change_counter);

        assert!(file.storage_dir_path().exists());
        assert!(file.custom_metadata_path().exists());
        assert!(file.symlink_dir_path().exists());
    }

    #[test]
    fn test_real_file_exists() {
        let (file_handler_config, mut change_counter) = rm_dirs_ce_dirs_get_default_helpers();

        // create a file in the symlink directory
        let real_file_in_symlink = &file_handler_config.symlink_directory.join("file.txt");
        fs::write(&real_file_in_symlink, "Hello World!").unwrap();

        // create a "duplicate file in the storage directory"
        let real_file_in_storage = &file_handler_config.storage_directory.join("file.txt");
        fs::write(&real_file_in_storage, "Hello World!").unwrap();

        // create another duplicate
        let real_file_in_storage = &file_handler_config.storage_directory.join("file (1).txt");
        fs::write(&real_file_in_storage, "Hello World!").unwrap();

        // call the function we are testing
        let file = client_database::FilePaths::from_path_checked(
            real_file_in_symlink.clone(),
            &file_handler_config,
        )
        .unwrap();

        real_file_exists(&file, &file_handler_config, &mut change_counter);

        let unique_file = client_database::FilePaths::from_path_checked(
            file_handler_config.storage_directory.join("file (2).txt"),
            &file_handler_config,
        );

        assert!(unique_file.is_ok());
        let unique_file = unique_file.unwrap();
        assert!(unique_file.storage_dir_path().exists());
        assert!(unique_file.custom_metadata_path().exists());
    }

    #[test]
    fn test_symlink_points_to_storage_dir() {
        let (file_handler_config, mut change_counter) = rm_dirs_ce_dirs_get_default_helpers();

        // create a file in the storage directory
        let storage_file = &file_handler_config.storage_directory.join("file.txt");
        fs::write(&storage_file, "Hello World!").unwrap();

        // create a custom metadata file
        let custom_metadata_file = client_database::relative_to_custom_metadata(
            &path::PathBuf::from("file.txt"),
            &file_handler_config,
        );
        fs::write(&custom_metadata_file, "some metadata").unwrap();

        // make a symlink in a different location that points to the storage file (simulate a move)
        let symlink_file = &file_handler_config.symlink_directory.join("file1.txt");
        symlink::symlink_file(&storage_file, &symlink_file).unwrap();

        // call the function we are testing
        let file = client_database::FilePaths::from_path_checked(
            symlink_file.clone(),
            &file_handler_config,
        )
        .unwrap();

        symlink_points_to_storage_dir(&file, &file_handler_config, &mut change_counter);

        assert!(fs::read_link(&file.symlink_dir_path()).is_ok());
        assert!(file.storage_dir_path().exists());
        assert!(file.custom_metadata_path().exists());

        // now delete the storage file (file1.txt - simulate a delete)
        let new_storage_file = &file_handler_config.storage_directory.join("file1.txt");
        fs::remove_file(&new_storage_file).unwrap();

        // call the function we are testing
        let new_file = client_database::FilePaths::from_path_checked(
            file_handler_config.symlink_directory.join("file1.txt"),
            &file_handler_config,
        )
        .unwrap();
        symlink_points_to_storage_dir(&new_file, &file_handler_config, &mut change_counter);

        assert!(!new_file.storage_dir_path().exists());
        assert!(!new_file.custom_metadata_path().exists());
        assert!(!new_file.symlink_dir_path().exists());
    }
}
