#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::App;
use clap::AppSettings;

extern crate dockup_utils;
use dockup_utils::config as config;
use dockup_utils::program as program;
use dockup_utils::logger as logger;

use std::error::Error;

fn main() {
    //Init the logger
    match logger::init() {
        Err(why) => panic!("couldn't init logger: {}, caused by {:?}",  why.description(), why.cause()),
        Ok(_) => println!("logger initialised"),
    }

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("resources/dockup_cli.yaml");
    let matches = App::from_yaml(yaml).setting(AppSettings::SubcommandRequired).get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {

        let config_path = matches.value_of("config").unwrap_or(config::DOCKUP_CONFIG_FILENAME);

        program::install::execute(config_path);
    }
}