use std::{
    collections::{HashMap, HashSet},
    path,
    sync::{Arc, Mutex},
    thread, time,
};

use log::error;

use super::Data;

pub fn possible_delete(
    tracking_id: usize,
    event_tracking: Arc<Mutex<HashMap<usize, path::PathBuf>>>,
    data: &Data,
) {
    let d = data.clone();
    let _ = thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(10));
        let mut e = event_tracking.lock().unwrap();
        if let Some(delete_path) = e.get(&tracking_id) {
            handle_delete(delete_path, &d);
            e.remove(&tracking_id).unwrap();
        };
    });
}

pub fn clear_item(path: &path::PathBuf, program_events: Arc<Mutex<HashSet<path::PathBuf>>>) {
    let p = path.clone();
    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(10));
        program_events.lock().unwrap().remove(&p);
    });
}

pub fn handle_create(
    path: &path::PathBuf,
    is_dir: bool,
    program_events: Arc<Mutex<HashSet<path::PathBuf>>>,
    data: &Data,
) {
    let mut events = program_events.lock().unwrap();
    if events.contains(path) {
        return;
    };

    let rel_path = relative_path(&data.0.symlink_directory, path).unwrap();
    events.insert(path.clone());
    drop(events);

    match event_create(&rel_path, is_dir, FType::Symlink, data, true) {
        Ok(_) => (),
        Err(e) => error!("Failed to make create change. live/run.rs: {e}"),
    };
    clear_item(path, program_events);
}

pub fn handle_move(
    from_path: &path::PathBuf,
    to_path: &path::PathBuf,
    program_events: Arc<Mutex<HashSet<path::PathBuf>>>,
    data: &Data,
) {
    let mut events = program_events.lock().unwrap();
    if events.contains(to_path) {
        return;
    };

    let rel_from_path = relative_path(&data.0.symlink_directory, from_path).unwrap();
    let rel_to_path = relative_path(&data.0.symlink_directory, to_path).unwrap();
    events.insert(to_path.clone());
    drop(events);

    match event_move(&rel_from_path, &rel_to_path, FType::Symlink, data, true) {
        Ok(_) => (),
        Err(e) => error!("Failed to make modify change. live/run.rs: {e}"),
    };
    clear_item(to_path, program_events);
}

pub fn handle_modify(
    parent_dir: &path::PathBuf,
    path: &path::PathBuf,
    program_events: Arc<Mutex<HashSet<path::PathBuf>>>,
    data: &Data,
) {
    let mut events = program_events.lock().unwrap();
    if events.contains(path) {
        return;
    };

    let rel_path = relative_path(parent_dir, path).unwrap();
    events.insert(path.clone());
    drop(events);

    match event_modify(&rel_path, path.is_dir(), data, true) {
        Ok(_) => (),
        Err(e) => error!("Failed to make modify change. live/run.rs: {e}"),
    };
    clear_item(path, program_events);
}

pub fn handle_delete(path: &path::PathBuf, data: &Data) {
    if path.is_symlink() {
        return;
    }
    let x = client_database::FilePaths::
    let rel_path = relative_path(&data.0.symlink_directory, path).unwrap();
    let is_dir = data.0.storage_directory.join(&rel_path).is_dir();
    match event_delete(&rel_path, is_dir, data, true) {
        Ok(_) => (),
        Err(e) => error!("Failed to make delete change. live/run.rs: {e}"),
    };
}
