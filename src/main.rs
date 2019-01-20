extern crate env_logger;
#[macro_use]
extern crate log;
extern crate notify;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::stdin;
use std::sync::mpsc::channel;
use std::thread;

use env_logger::Env;

use crate::config_parser::build_config;
use crate::watcher::setup_watch;

mod watcher;
mod launcher;
mod config_parser;


fn main() {
    let env = Env::default().filter_or("RGR_LOG_LEVEL", "debug");
    env_logger::Builder::from_env(env).init();

    let config = build_config("config.json");

    let (tx, rx) = channel();

    let watch_thread = thread::spawn(move || {
        setup_watch(rx, config);
    });

    println!("Press any key to stop!");

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    tx.send(true).unwrap();
    watch_thread.join().unwrap();
}
