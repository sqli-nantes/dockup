
use super::ProgramDefinition;

use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub const DOCKUP_CONFIG_FILENAME: &'static str = "dockup.yaml";

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

//Load the content of the filename given as a slice into a ProgramDefinition struct
pub fn load_config_struct(filename: &str) -> ProgramDefinition {
    let configfile_content = read_file(filename);

    ProgramDefinition::from_yaml(&configfile_content)

}