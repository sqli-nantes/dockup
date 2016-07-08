extern crate yaml_rust;
use yaml_rust::{YamlLoader};

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;
//TODO : log
//https://doc.rust-lang.org/log/log/index.html

const DOCKUP_CONFIG_FILENAME: &'static str = "dockup.yaml";

struct ProgramDefinition<'a> {
    pub name: &'a str,
    pub cmd: &'a str,
    pub os: &'a Vec<&'a str>,
}

impl<'a> fmt::Display for ProgramDefinition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your program loaded  attributes :\n - name: {}\n - command : {}\n - targeted os: {:?}\n ", self.name, self.cmd, self.os)

    }
}

fn read_config_file(filename: &str) -> String {
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
        Ok(_) => println!("{} contains:\n{}", display, s),
    }

    s
}

fn main() {
    let configfile = read_config_file(DOCKUP_CONFIG_FILENAME);

    let docs = YamlLoader::load_from_str(&configfile).unwrap();
    let doc = &docs[0]; // select the first document

    //load the program struct
    let programdef = ProgramDefinition {
        name: doc["name"].as_str().unwrap(),
        cmd: doc["command"].as_str().unwrap(),
        os: &(doc["os"].as_vec().unwrap().iter()
                .map(|os|  {
                    os.as_str().unwrap()
                }).collect()),
    };


    println!("{}", programdef);
}