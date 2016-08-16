#[macro_use]
extern crate log;

extern crate dockup_utils;
use dockup_utils::config::load as load;
use dockup_utils::config::model::ProgramDefinition  as ProgramDefinition;
use dockup_utils::program::generate as generate;
use dockup_utils::logger;


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