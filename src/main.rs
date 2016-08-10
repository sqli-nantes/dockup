#[macro_use]
extern crate log;
use dockup::logger;

extern crate dockup;
use dockup::config::load;
use dockup::config::model::ProgramDefinition;
use dockup::program::generate;

use std::error::Error;

fn main() {
    match logger::init() {
        Err(why) => panic!("couldn't init logger: {}, caused by {:?}",  why.description(), why.cause()),
        Ok(_) => println!("logger initialised"),
    }

    let programdef: ProgramDefinition = load::load_config_struct(load::DOCKUP_CONFIG_FILENAME);

    info!("Loaded configuration : \n {}, {}", programdef, "test");

    info!("Generate content file");

    //Example of writing file
    generate::generate_program_source_file(&programdef.name,&programdef.cmd);
}