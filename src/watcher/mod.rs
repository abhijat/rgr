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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;
    use std::path;
    use std::thread;

    use crate::config_parser::CommandConfig;

    use super::*;

    const GEN_FILE: &str = "./sample_files/test_data/generated/x";

    fn test_config() -> WatcherConfig {
        WatcherConfig {
            command_config: CommandConfig {
                binary_path: "touch".to_string(),
                args: vec![GEN_FILE.to_string()],
            },
            debounce_in_seconds: 1,
            path_to_watch: "./sample_files/shell_files/".to_string(),
            file_extension_to_watch_for: Some(".sh".to_string()),
        }
    }

    fn write_to_file() {
        let mut f = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./sample_files/shell_files/foo.sh")
            .expect("!open_file");

        f.write(&[1]).unwrap();
    }

    fn setup() {
        let test_path = "./sample_files/test_data/generated";
        let watch_path = "./sample_files/shell_files/";

        fs::create_dir_all(test_path).expect("!create_dir_all");
        fs::create_dir_all(watch_path).expect("!create_dir_all");

        if path::Path::new(GEN_FILE).exists() {
            fs::remove_file(GEN_FILE).expect("!delete");
        }
    }

    fn teardown() {
        fs::remove_file(GEN_FILE).expect("!remove_file");
    }

    #[test]
    fn setup_watch_test() {
        setup();

        let cfg = test_config();
        let (tx, rx) = channel();

        // Launch write_to_file after 1 second delay
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(1));
            write_to_file();
        });

        // The shutdown task after a 3 second delay
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(3));
            tx.send(true).unwrap();
        });

        // Start the watch
        setup_watch(rx, cfg);
        assert!(path::Path::new(GEN_FILE).exists());

        teardown();
    }
}
