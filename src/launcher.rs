use std::process::Command;
use std::process::Stdio;

use crate::config_parser::WatcherConfig;

pub fn launch_command(config: &WatcherConfig) {
    let config = &config.command_config;
    let mut command = Command::new(config.binary_path.as_str());

    command.args(&config.args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    debug!("{:?}", command);
    let mut process = command.spawn()
        .expect("!spawn");

    process.wait().expect("!wait");
}
