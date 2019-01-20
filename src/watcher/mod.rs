use std::collections;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

use notify::{DebouncedEvent, RecommendedWatcher, Watcher};

use crate::config_parser::WatcherConfig;
use crate::watcher::file_change_actions::react_to_file_change;
use crate::watcher::path_collection_actions::add_paths_to_inotify_watcher;
use crate::watcher::path_collection_actions::build_watch_paths;

mod path_collection_actions;
mod file_change_actions;


pub fn setup_watch(shutdown_channel: Receiver<bool>, config: WatcherConfig) {
    let (tx, rx) = channel();
    let debounce = Duration::from_secs(config.debounce_in_seconds as u64);
    let mut watcher: RecommendedWatcher = Watcher::new(tx, debounce)
        .expect("!!watcher_new");

    let extension = config.extension();

    let mut paths = collections::HashSet::new();
    build_watch_paths(
        config.path_to_watch.as_str(),
        &mut paths,
        extension.as_str(),
    ).expect("failed to build watch paths");

    info!("added {} paths to watch list", paths.len());

    add_paths_to_inotify_watcher(paths, &mut watcher);

    let extension = extension.as_str();
    let filter = |s: &str| {
        s.ends_with(extension)
    };

    let loop_interval = Duration::from_millis(100);
    loop {
        match shutdown_channel.recv_timeout(loop_interval) {
            Ok(_) => {
                debug!("received stop signal! shutting down");
                return;
            }

            Err(RecvTimeoutError::Timeout) => {}

            Err(err) => {
                error!("error while listening on shutdown channel{}", err);
            }
        }

        match rx.recv_timeout(loop_interval) {
            Ok(ref event) => {
                match event {
                    DebouncedEvent::Create(filename) | DebouncedEvent::Write(filename) | DebouncedEvent::Remove(filename) => {
                        react_to_file_change(&filename.to_string_lossy(), &filter, &config);
                    }
                    _ => {}
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(err) => {
                error!("failed while reading from inotify: {}", err);
            }
        }
    }
}
