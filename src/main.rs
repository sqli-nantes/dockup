extern crate dockup;
use dockup::config::load;
use dockup::config::model::ProgramDefinition;

#[macro_use]
extern crate log;
use dockup::logger;

use std::error::Error;

fn main() {
    match logger::init() {
        Err(why) => panic!("couldn't init logger: {}, caused by {:?}",  why.description(), why.cause()),
        Ok(_) => println!("logger initialised"),
    }

    let programdef: ProgramDefinition = load::load_config_struct(load::DOCKUP_CONFIG_FILENAME);

    info!("Loaded configuration : \n {}", programdef);
}