extern crate yaml_rust;
use self::yaml_rust::YamlLoader;

use super::model::ProgramDefinition;

use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub const DOCKUP_CONFIG_FILENAME: &'static str = "dockup.yaml";
const DOCKUP_CONFIG_ATTR_NAME: &'static str = "name";
const DOCKUP_CONFIG_ATTR_COMMAND: &'static str = "command";
const DOCKUP_CONFIG_ATTR_OS: &'static str = "os";

//Read the configuration file add returns a String
fn read_file(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };


    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) =>  info!("{} contains:\n{}", display, s),
    }

    s
}

// Load the Yaml given as a string content into a ProgramDefinition struct
fn load_yaml_to_model(yaml_str: &str) -> ProgramDefinition {
    let docs = YamlLoader::load_from_str(yaml_str).unwrap();
    let doc = &docs[0]; // select the first document

    //Destruct the yaml content
    let mut name_string = String::new();
    name_string.push_str(doc[DOCKUP_CONFIG_ATTR_NAME].as_str().unwrap());
    let mut command_string = String::new();
    command_string.push_str(doc[DOCKUP_CONFIG_ATTR_COMMAND].as_str().unwrap());
    let os_vec = doc[DOCKUP_CONFIG_ATTR_OS].as_vec().unwrap().iter()
        .map(|os| {
            let mut result_string = String::new();
            result_string.push_str(os.as_str().unwrap());

            result_string
        }).collect();

    //load the program struct from each loaded elements
    let programdef = ProgramDefinition {
        name: name_string,
        cmd: command_string,
        os: os_vec,
    };

    programdef
}

//Load the content of the filename given as a slice into a ProgramDefinition struct
pub fn load_config_struct(filename: &str) -> ProgramDefinition {
    let configfile = read_file(filename);
    load_yaml_to_model(&configfile)
}