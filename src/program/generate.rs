use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

extern crate rustache;

const RUST_SOURCE_EXT: &'static str = "rs";
const DOCKUP_SOURCE_TEMPLATE_FILENAME: &'static str = "src/resources/program_call.tmpl";

//Write the program source
fn write_file(filename: &str, content: String) {

    let path = Path::new(filename);
    let display = path.display();

    // Create a file, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    // Write the content string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => info!("successfully wrote to {}", display),
    }

}

fn generate_content(command: &str) -> String {


    let data = rustache::HashBuilder::new().insert_string("command", command);
    let rv = rustache::render_file(DOCKUP_SOURCE_TEMPLATE_FILENAME, data);

    let mut buffer = Vec::new();

    match rv.unwrap().read_to_end(&mut buffer) {
        Err(why) => {
            panic!("couldn't read {} file because {}", DOCKUP_SOURCE_TEMPLATE_FILENAME,
                                               why.description())
        },
        Ok(_) => String::from_utf8(buffer).unwrap()
    }

}

pub fn generate_program_source_file(filename: &str, command: &str){

    let content: String = generate_content(command);
    let complete_filename = format!("{}.{}",filename,RUST_SOURCE_EXT);

    write_file(&complete_filename, content);
}