use std::collections;
use std::fs;
use std::path::PathBuf;
use notify::RecursiveMode;
use notify::Watcher;

pub fn build_watch_paths(root_path: &str, paths: &mut collections::HashSet<PathBuf>, extension: &str) {
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

pub fn add_paths_to_inotify_watcher<W>(paths: collections::HashSet<PathBuf>, watcher: &mut W)
    where W: Watcher {
    for path in paths {
        if let Err(err) = watcher.watch(path.clone(), RecursiveMode::NonRecursive) {
            error!("failed to add watch for {} with error {}", path.to_string_lossy(), err);
        }
    }
}
