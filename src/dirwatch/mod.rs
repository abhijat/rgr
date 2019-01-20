use std::collections;
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

use crate::config_parser::WatcherConfig;
use crate::dirwatch::build_paths::build_watch_paths;
use crate::dirwatch::change_actions::react_to_file_change;

mod build_paths;
mod change_actions;

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
