use std::process::Command;
use std::io::Read;
use std::error::Error;

extern crate rustache;

use super::ProgramDefinition;

pub mod install;
pub mod run;


fn create_command_from_programdef(programdef : ProgramDefinition) -> Command {
    let command_str = String::from(programdef.command);
    //Make a vec of all arguments included the command binary
    let command_chunks:Vec<&str> = command_str.split(' ').collect();

    //Separate the command binary from the args
    let command_name_args = command_chunks.split_first();

    let mut command = Command::new(command_name_args.unwrap().0);
    command.args(&command_name_args.unwrap().1);

    command
}

fn generate_executable_content(config_path: &str) -> String  {

    let template_str = include_str!("../resources/exe_dockup.tmpl");
    let data = rustache::HashBuilder::new().insert_string("config_path", config_path);
    let rv = rustache::render_text(template_str, data);

    let mut buffer = Vec::new();

    match rv.unwrap().read_to_end(&mut buffer) {
        Err(why) => {
            panic!("couldn't read {} file because {}", "../resources/exe_dockup.tmpl",
                                               why.description())
        },
        Ok(_) => String::from_utf8(buffer).unwrap()
    }

}