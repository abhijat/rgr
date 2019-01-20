use std::collections;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::config_parser::WatcherConfig;
use crate::launcher::launch_command;

fn build_watch_paths(root_path: &str, paths: &mut collections::HashSet<PathBuf>, extension: &str) {
    let rd_dir = fs::read_dir(root_path).expect("!!read_dir");

    for entry in rd_dir {
        let entry = entry.expect("!!file_entry_read");

        let path = entry.path();
        let metadata = fs::metadata(&path).expect("!!metadata");

        if metadata.is_file() && entry.file_name().to_string_lossy().ends_with(extension) {
            let parent = path.parent().expect("!!parent");

            debug!("adding {:?} to watch list", parent);
            paths.insert(PathBuf::from(parent));
        }

        if metadata.is_dir() {
            build_watch_paths(&entry.path().to_string_lossy(), paths, extension);
        }
    }
}

fn react_to_file_change(filename: &str, filter: &Fn(&str) -> bool, config: &WatcherConfig) {
    if filter(filename) {
        launch_command(config);
    }
}

pub fn setup_watch(config: WatcherConfig) {
    let (tx, rx) = channel();
    let debounce = Duration::from_secs(config.debounce_in_seconds as u64);
    let mut watcher: RecommendedWatcher = Watcher::new(tx, debounce)
        .expect("!!watcher_new");

    let extension = config.extension();

    let mut paths = collections::HashSet::new();
    build_watch_paths(config.path_to_watch.as_str(), &mut paths, extension.as_str());

    info!("added {} paths to watch list", paths.len());

    for path in paths {
        if let Err(err) = watcher.watch(path.clone(), RecursiveMode::NonRecursive) {
            error!("failed to add watch for {} with error {}", path.to_string_lossy(), err);
        }
    }

    let extension = extension.as_str();
    let filter = |s: &str| {
        s.ends_with(extension)
    };

    loop {
        match rx.recv() {
            Ok(ref event) => {
                match event {
                    DebouncedEvent::Create(filename) | DebouncedEvent::Write(filename) | DebouncedEvent::Remove(filename) => {
                        react_to_file_change(&filename.to_string_lossy(), &filter, &config);
                    }
                    _ => {}
                }
            }
            Err(err) => {
                error!("failed while reading from inotify: {}", err);
            }
        }
    }
}
