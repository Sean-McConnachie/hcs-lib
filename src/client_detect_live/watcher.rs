use notify::Watcher;
use std::path;

pub fn make_watcher(
    path: &path::PathBuf,
    tx: std::sync::mpsc::Sender<notify::Result<notify::Event>>,
) -> notify::INotifyWatcher {
    let mut watcher = notify::RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
    watcher
        .watch(&path, notify::RecursiveMode::Recursive)
        .unwrap();
    watcher
}
