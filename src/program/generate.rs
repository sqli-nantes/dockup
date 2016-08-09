use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


//Write the program source
pub fn write_file(filename: &str, content: String) {

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