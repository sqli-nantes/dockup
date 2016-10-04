
use super::ProgramDefinition;

use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
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
        Err(why) => panic!("couldn't open the file {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };


    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) =>  debug!("{} contains:\n{}", display, s),
    }

    s
}

fn write_file(filepath: &str, content: &str) {
    let path = Path::new(filepath);
    let display = path.display();

    // Create the file, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't create the file {}: {}", filepath,
                                                   why.description()),
        Ok(file) => file,
    };

    // Write the content string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn save_config_struct(config_dirpath: &str, programdef: &ProgramDefinition) {
    info!("Save config file !");
    let yaml_str = ProgramDefinition::to_yaml(&programdef);

    //concatenate dirpath and default config filename
    let mut filepath = PathBuf::from(config_dirpath);
    filepath.push(DOCKUP_CONFIG_FILENAME);

    //write yaml to the dockup install file
    write_file(& filepath.to_str().unwrap(), &yaml_str);

}

//Load the content of the filename given as a slice into a ProgramDefinition struct
pub fn load_config_struct(filename: &str) -> ProgramDefinition {
    let configfile_content = read_file(filename);

    ProgramDefinition::from_yaml(&configfile_content)

}