use crate::config_parser::WatcherConfig;
use crate::launcher::launch_command;

pub fn react_to_file_change(filename: &str, filter: &Fn(&str) -> bool, config: &WatcherConfig) {
    if filter(filename) {
        info!("reacting to change for file: {}", filename);
        launch_command(config);
        info!("command execution finished");
    }
}
