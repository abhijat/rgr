extern crate clap;
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
use std::time::Duration;

use clap::App;
use clap::Arg;
use env_logger::Env;

use crate::config_parser::build_config;
use crate::watcher::setup_watch;

mod watcher;
mod launcher;
mod config_parser;


fn main() {
    let matches = App::new("Red-Green-Refactor")
        .version("0.1.0")
        .author("Abhijat Malviya <malviya.abhijat@gmail.com>")
        .about("React to file changes by running commands")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .help("path to config file")
            .takes_value(true))
        .get_matches();

    let env = Env::default().filter_or("RGR_LOG_LEVEL", "debug");
    env_logger::Builder::from_env(env).init();

    let config_path = matches.value_of("config").unwrap_or("./config.json");

    let config = build_config(config_path);

    let (tx, rx) = channel();

    let watch_thread = thread::spawn(move || {
        setup_watch(rx, config);
    });

    thread::sleep(Duration::from_secs(1));
    println!("Press any key to stop!");

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("!read_line");

    tx.send(true).expect("!send");
    watch_thread.join().expect("!join");
}
