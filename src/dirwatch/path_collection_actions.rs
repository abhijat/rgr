use std::collections;
use std::fs;
use std::io;
use std::path::PathBuf;

use notify::RecursiveMode;
use notify::Watcher;

pub fn build_watch_paths(
    root_path: &str,
    paths: &mut collections::HashSet<PathBuf>,
    extension: &str,
) -> io::Result<()> {
    let rd_dir = fs::read_dir(root_path)?;

    for entry in rd_dir {
        let entry = entry?;

        let path = entry.path();
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() && entry.file_name().to_string_lossy().ends_with(extension) {
            let parent = path.parent()
                .ok_or(io::Error::new(io::ErrorKind::Other, "failed to acquire parent"))?;

            debug!("adding {:?} to watch list", parent);
            paths.insert(PathBuf::from(parent));
        }

        if metadata.is_dir() {
            build_watch_paths(&entry.path().to_string_lossy(), paths, extension)?;
        }
    }

    Ok(())
}

pub fn add_paths_to_inotify_watcher<W>(paths: collections::HashSet<PathBuf>, watcher: &mut W)
    where W: Watcher {
    for path in paths {

        // TODO mode needs to be user-definable

        if let Err(err) = watcher.watch(path.clone(), RecursiveMode::NonRecursive) {
            error!("failed to add watch for {} with error {}", path.to_string_lossy(), err);
        }
    }
}
