use std::{fs, path};

use log::debug;

use crate::client_database;

pub fn move_file(
    rel_from_path: &path::PathBuf,
    rel_to_path: &path::PathBuf,
    file_handler_config: &client_database::FileHandlerConfig,
    change_counter: &mut client_database::ChangeCounter,
    skip_move: client_database::Type,
) {
    debug!(
        "`move_file` - from: `{}` to: `{}`",
        rel_from_path.display(),
        rel_to_path.display()
    );
    if skip_move != client_database::Type::Symlink {
        // Move symlink (delete, then create)
        let symlink_from_path = file_handler_config.symlink_directory.join(rel_from_path);
        let symlink_to_path = file_handler_config.symlink_directory.join(rel_to_path);

        if symlink_from_path.exists() {
            symlink::remove_symlink_file(&symlink_from_path).unwrap();
        }
        symlink::symlink_file(&symlink_to_path, &symlink_from_path).unwrap();
    }

    if skip_move != client_database::Type::File {
        // Move real file (rename)
        let real_from_path = file_handler_config.storage_directory.join(rel_from_path);
        let real_to_path = file_handler_config.storage_directory.join(rel_to_path);

        if real_from_path.exists() {
            fs::rename(&real_from_path, &real_to_path).unwrap();
        }
    }

    if skip_move != client_database::Type::CustomMetadata {
        // Move custom metadata file (rename)
        let custom_metadata_from_path =
            client_database::relative_to_custom_metadata(rel_from_path, file_handler_config);
        let custom_metadata_to_path =
            client_database::relative_to_custom_metadata(rel_to_path, file_handler_config);

        if custom_metadata_from_path.exists() {
            fs::rename(&custom_metadata_from_path, &custom_metadata_to_path).unwrap();
        }
    }

    {
        // Add file move change
        let move_change = format!(
            "file_move\n{}\n{}",
            rel_from_path.to_str().unwrap(),
            rel_to_path.to_str().unwrap()
        );
        fs::write(change_counter.next_path(), move_change).unwrap();
    }
}
