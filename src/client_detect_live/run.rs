use std::{
    collections::{HashMap, HashSet},
    ffi, path,
    sync::{Arc, Mutex},
};

use log::{error, info};

use crate::client_database;

use super::{events, watcher::make_watcher};

pub fn run_live(file_handler_config: &client_database::FileHandlerConfig) {
    info!("Running live detect");

    let mut change_counter =
        client_database::ChangeCounter::init(&file_handler_config.program_data_directory);
    let data = (file_handler_config, &mut change_counter);

    let (tx, rx) = std::sync::mpsc::channel();

    info!("Watching {:?}", &file_handler_config.symlink_directory);
    let mut _shortcut_watcher = make_watcher(&file_handler_config.symlink_directory, tx.clone());

    info!("Watching {:?}", &file_handler_config.storage_directory);
    let mut _shortcut_watcher = make_watcher(&file_handler_config.storage_directory, tx.clone());

    let event_tracking = Arc::new(Mutex::new(HashMap::new()));
    let program_events = Arc::new(Mutex::new(HashSet::new()));
    for res in rx {
        match handle_event(res, event_tracking.clone(), program_events.clone(), &data) {
            Ok(_) => (),
            Err(e) => error!("Failed to handle event: {e}"),
        };
    }
}

fn handle_event(
    event: notify::Result<notify::Event>,
    event_tracking: Arc<Mutex<HashMap<usize, path::PathBuf>>>,
    program_events: Arc<Mutex<HashSet<path::PathBuf>>>,
    data: &super::Data,
) -> anyhow::Result<()> {
    let event = event?;

    let in_shortcut = event.paths[0].starts_with(&data.0.symlink_directory);

    match (in_shortcut, event.kind) {
        (true, notify::EventKind::Create(c)) => {
            let path = &event.paths[0];
            match c {
                notify::event::CreateKind::File => {
                    if let Some(e) = path.extension() {
                        if e != ffi::OsStr::new("part") {
                            events::handle_create(&path, false, program_events, data)
                        }
                    }
                }
                notify::event::CreateKind::Folder => {
                    events::handle_create(&path, true, program_events, data)
                }
                _ => (),
            }
        }
        (true, notify::EventKind::Modify(m)) => {
            if let notify::event::ModifyKind::Name(n) = m {
                match n {
                    notify::event::RenameMode::From => {
                        let from_path = &event.paths[0];

                        if !from_path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .starts_with(".goutputstream")
                        {
                            let tracking_id = event.attrs.tracker().unwrap();
                            event_tracking
                                .lock()
                                .unwrap()
                                .insert(tracking_id, from_path.clone());

                            events::possible_delete(tracking_id, event_tracking, data)
                        }
                    }
                    notify::event::RenameMode::Both => {
                        let tracking_id = event.attrs.tracker().unwrap();
                        event_tracking.lock().unwrap().remove(&tracking_id);

                        let from_path = &event.paths[0];
                        let to_path = &event.paths[1];
                        if from_path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .starts_with(".goutputstream")
                        {
                            events::handle_modify(
                                &data.0.symlink_directory.clone(),
                                to_path,
                                program_events,
                                data,
                            );
                        } else {
                            if let Some(e) = from_path.extension() {
                                if e == ffi::OsStr::new("part") {
                                    events::handle_create(to_path, false, program_events, data)
                                } else {
                                    events::handle_move(from_path, to_path, program_events, data)
                                }
                            } else {
                                events::handle_move(from_path, to_path, program_events, data)
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
        (false, notify::EventKind::Access(m)) => {
            if let notify::event::AccessKind::Close(n) = m {
                if matches!(n, notify::event::AccessMode::Write) {
                    let path = &event.paths[0];
                    if !client_database::CustomMetadataType::is_custom_metadata(path) {
                        events::handle_modify(
                            &data.0.storage_directory.clone(),
                            path,
                            program_events,
                            data,
                        );
                    }
                }
            }
        }
        _ => (),
    }

    Ok(())
}
