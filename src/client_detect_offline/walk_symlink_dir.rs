use std::fs;

use crate::client_database;

pub fn walk_symlink(file_handler_config: &client_database::FileHandlerConfig) {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let file_paths = {
            let path_buf = path.unwrap().path().to_path_buf();
            client_database::FilePaths::from_path(path_buf, &file_handler_config).unwrap()
        };

        match file_paths.current_type() {
            client_database::FileType::Real => {
                // Case covered by `RealInSymlinkDir`.
                unreachable!();
            }
            client_database::FileType::CustomMetadata => {
                // Case 6
                todo!();
            }
            client_database::FileType::Symlink => {
                // Case 1/2/3
                todo!();
            }
            client_database::FileType::RealInSymlinkDir => {
                // Case 4
                todo!();
            }
        }
    }

    walk_symlink(file_handler_config);
}
