use std::fs;

use crate::client_database;

/// Case 7
pub fn real_file_exists(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    {
        if !(file.custom_metadata_path().exists() && fs::read_link(file.symlink_dir_path()).is_ok())
        {
            client_database::change_events::delete_file(file, change_counter);
            return;
        }
    }

    {
        // Check if last modified of file is the same as the last modified contained in the custom metadata
        let file_last_modified =
            client_database::CustomMetadata::last_modified_of_file(&file.storage_dir_path())
                .unwrap();
        let custom_metadata = client_database::CustomMetadata::read_from_file(&file).unwrap();

        if file_last_modified != custom_metadata.last_modified() {
            client_database::change_events::modify_file(file, change_counter);
        }
    }
}

/// Case 8
pub fn custom_metadata_exists(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    let is_dir = if file.storage_dir_path().exists() {
        if file.storage_dir_path().is_dir() {
            true
        } else {
            false
        }
    } else if fs::read_link(&file.symlink_dir_path()).is_ok() {
        false
    } else if file.symlink_dir_path().exists() && file.symlink_dir_path().is_dir() {
        true
    } else {
        // THIS AIN'T GOOD
        false
    };

    if is_dir && !(file.symlink_dir_path().exists() && file.storage_dir_path().exists()) {
        client_database::change_events::delete_dir(file, change_counter);
    } else if !is_dir
        && !(file.symlink_dir_path().exists() && fs::read_link(&file.symlink_dir_path()).is_ok())
    {
        client_database::change_events::delete_file(file, change_counter)
    }
}

/// Case 9
pub fn directory_exists(
    file: &client_database::FilePaths,
    change_counter: &mut client_database::ChangeCounter,
) {
    if !(file.custom_metadata_path().exists() && file.symlink_dir_path().exists()) {
        client_database::change_events::delete_dir(file, change_counter);
        return;
    }

    {
        let directory_last_modified =
            client_database::CustomMetadata::last_modified_of_file(&file.symlink_dir_path())
                .unwrap();
        let custom_metadata = client_database::CustomMetadata::read_from_file(&file).unwrap();

        if directory_last_modified != custom_metadata.last_modified() {
            client_database::change_events::modify_dir(file, change_counter);
        }
    }
}

/// Case 10
pub fn symlink_exists() {
    // Ignore the file
    return;
}

#[cfg(test)]
mod test {
    use std::{fs, path};

    use crate::{client_database, testing_utils::rm_dirs_ce_dirs_get_default_helpers};

    use super::*;

    #[test]
    fn test_real_file_exists() {
        let (file_handler_config, mut change_counter) = rm_dirs_ce_dirs_get_default_helpers();

        let real_fp = client_database::relative_to_real(
            &path::PathBuf::from("file.txt"),
            &file_handler_config,
        );
        let _custom_metadata_fp = client_database::relative_to_custom_metadata(
            &path::PathBuf::from("file.txt"),
            &file_handler_config,
        );
        let symlink_path = client_database::relative_to_symlink(
            &path::PathBuf::from("file.txt"),
            &file_handler_config,
        );

        // create real_file
        fs::write(&real_fp, "Hello, world!").unwrap();

        // create symlink
        symlink::symlink_file(&real_fp, &symlink_path).unwrap();

        let file = client_database::FilePaths::from_path_checked(
            symlink_path.clone(),
            &file_handler_config,
        )
        .unwrap();

        // create custom_metadata
        let custom_metadata = client_database::CustomMetadata::new(
            client_database::CustomMetadata::last_modified_of_file(&real_fp).unwrap(),
        );
        custom_metadata.write_to_file(&file).unwrap();

        // call function
        real_file_exists(&file, &mut change_counter);

        assert!(file.custom_metadata_path().exists());
        assert!(file.symlink_dir_path().exists());
        assert!(file.storage_dir_path().exists());
        assert!(change_counter.change_count() == 0);

        // remove custom_metadata
        fs::remove_file(file.custom_metadata_path()).unwrap();

        // call function
        real_file_exists(&file, &mut change_counter);

        assert!(!file.custom_metadata_path().exists());
        assert!(!file.symlink_dir_path().exists());
        assert!(!file.storage_dir_path().exists());
        assert!(change_counter.change_count() == 1);

        // create real_file
        fs::write(&real_fp, "Hello, world!").unwrap();

        // create symlink
        symlink::symlink_file(&real_fp, &symlink_path).unwrap();

        let file = client_database::FilePaths::from_path_checked(
            symlink_path.clone(),
            &file_handler_config,
        )
        .unwrap();

        // create custom_metadata
        let custom_metadata = client_database::CustomMetadata::new(0);
        custom_metadata.write_to_file(&file).unwrap();

        // call function
        real_file_exists(&file, &mut change_counter);

        assert!(file.custom_metadata_path().exists());
        assert!(file.symlink_dir_path().exists());
        assert!(file.storage_dir_path().exists());
        assert!(change_counter.change_count() == 2);
    }

    #[test]
    fn test_custom_metadata_exists() {
        let (file_handler_config, mut change_counter) = rm_dirs_ce_dirs_get_default_helpers();

        let real_fp = client_database::relative_to_real(
            &path::PathBuf::from("file.txt"),
            &file_handler_config,
        );

        let symlink_path = client_database::relative_to_symlink(
            &path::PathBuf::from("file.txt"),
            &file_handler_config,
        );

        // create real file
        fs::write(&real_fp, "Hello, world!").unwrap();

        // create symlink
        symlink::symlink_file(&real_fp, &symlink_path).unwrap();

        let file = client_database::FilePaths::from_path_checked(
            symlink_path.clone(),
            &file_handler_config,
        )
        .unwrap();

        // create custom_metadata
        let custom_metadata = client_database::CustomMetadata::new(
            client_database::CustomMetadata::last_modified_of_file(&real_fp).unwrap(),
        );
        custom_metadata.write_to_file(&file).unwrap();

        // call function
        custom_metadata_exists(&file, &mut change_counter);

        assert!(file.custom_metadata_path().exists());
        assert!(file.symlink_dir_path().exists());
        assert!(file.storage_dir_path().exists());
        assert!(change_counter.change_count() == 0);

        // remove symlink
        symlink::remove_symlink_file(&symlink_path).unwrap();

        // call function
        custom_metadata_exists(&file, &mut change_counter);

        assert!(!file.custom_metadata_path().exists());
        assert!(!file.symlink_dir_path().exists());
        assert!(!file.storage_dir_path().exists());
        assert!(change_counter.change_count() == 1);

        // repeat for a directory instead
        let real_fp =
            client_database::relative_to_real(&path::PathBuf::from("dir"), &file_handler_config);

        let symlink_path =
            client_database::relative_to_symlink(&path::PathBuf::from("dir"), &file_handler_config);

        // create real dir in storage
        fs::create_dir(&real_fp).unwrap();

        // create dir in symlink
        fs::create_dir(&symlink_path).unwrap();

        let file = client_database::FilePaths::from_path_checked(
            symlink_path.clone(),
            &file_handler_config,
        )
        .unwrap();

        // create custom_metadata
        let custom_metadata = client_database::CustomMetadata::new(
            client_database::CustomMetadata::last_modified_of_file(&real_fp).unwrap(),
        );

        custom_metadata.write_to_file(&file).unwrap();

        // call function
        custom_metadata_exists(&file, &mut change_counter);

        assert!(file.custom_metadata_path().exists());
        assert!(file.symlink_dir_path().exists());
        assert!(file.storage_dir_path().exists());
        assert!(change_counter.change_count() == 1);

        // remove dir in symlink
        fs::remove_dir(&symlink_path).unwrap();

        // call function
        custom_metadata_exists(&file, &mut change_counter);

        assert!(!file.custom_metadata_path().exists());
        assert!(!file.symlink_dir_path().exists());
        assert!(!file.storage_dir_path().exists());
        assert!(change_counter.change_count() == 2);
    }

    #[test]
    fn test_directory_exists() {}
}
