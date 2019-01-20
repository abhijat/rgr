extern crate env_logger;
#[macro_use]
extern crate log;
extern crate notify;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use env_logger::Env;

use crate::watcher::setup_watch;
use crate::config_parser::build_config;

mod watcher;
mod launcher;
mod config_parser;


fn main() {
    let env = Env::default().filter_or("RGR_LOG_LEVEL", "debug");
    env_logger::Builder::from_env(env).init();

    let config = build_config("config.json");
    setup_watch(config);
}
