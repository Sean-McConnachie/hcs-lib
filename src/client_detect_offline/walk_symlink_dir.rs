use std::{fs, path};

use log::debug;

use super::symlink_cases;
use crate::client_database;

/// Cases 1 -> 6 handled here. See `client_detect_offline` for an explanation of the cases.
pub fn walk_symlink(
    dir: &path::PathBuf,
    file_handler_config: &client_database::FileHandlerConfig,
    change_counter: &mut client_database::ChangeCounter,
) {
    debug!("Walking symlink directory: {:?}", dir);

    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let path_buf = path.unwrap().path().to_path_buf();
        let file_paths =
            client_database::FilePaths::from_path_checked(path_buf.clone(), &file_handler_config)
                .unwrap();

        match (file_paths.file_location(), file_paths.file_type()) {
            (client_database::FileLocation::SymlinkDir, client_database::Type::Symlink) => {
                // Case 1/2/3
                let points_to = client_database::FilePaths::from_path_checked(
                    file_paths.points_to().unwrap().clone(),
                    &file_handler_config,
                )
                .unwrap();
                match points_to.file_location() {
                    client_database::FileLocation::SymlinkDir => {
                        // Case 1
                        symlink_cases::symlink_points_to_symlink_dir(
                            &file_paths,
                            file_handler_config,
                        );
                    }
                    client_database::FileLocation::StorageDir => {
                        // Case 2
                        symlink_cases::symlink_points_to_storage_dir(
                            &file_paths,
                            file_handler_config,
                            change_counter,
                        );
                    }
                    client_database::FileLocation::OtherDir => {
                        // Case 3
                        symlink_cases::symlink_points_to_other_dir();
                    }
                }
            }
            (client_database::FileLocation::SymlinkDir, client_database::Type::File) => {
                // Case 4
                symlink_cases::real_file_exists(&file_paths, file_handler_config, change_counter);
            }
            (client_database::FileLocation::SymlinkDir, client_database::Type::Directory) => {
                // Case 5
                symlink_cases::directory_exists(&file_paths, change_counter);
                walk_symlink(file_paths.path(), file_handler_config, change_counter);
            }
            (client_database::FileLocation::SymlinkDir, client_database::Type::CustomMetadata) => {
                // Case 6
                symlink_cases::custom_metadata_exists(&path_buf);
            }
            (_, _) => unreachable!(
                "This should never happen as we are iterating through the symlink dir!"
            ),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_walk_symlink() {
        // let file_handler_config = client_database::FileHandlerConfig::new(
        //     "_storage_dir".to_string(),
        //     "_symlink_dir".to_string(),
        //     "_temporary_dir".to_string(),
        // );

        // walk_symlink(&file_handler_config.symlink_directory, &file_handler_config);
    }
}
