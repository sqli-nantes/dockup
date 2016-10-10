use std::error::Error;
use std::path::Path;
use std::fs::DirBuilder;
use std::io::prelude::*;
use std::fs::File;


pub fn create_dir(filepath: &str)  {
    debug!("Try to create config dir");
    let dir_path = Path::new(filepath);


    //TODO deal with not a directory but exists
    if !dir_path.exists() {
        info!("Create the directory {}", dir_path.display());
        match DirBuilder::new().recursive(true).create(dir_path.to_str().unwrap()) {
            Err(why) => panic!("couldn't create the directory {}: {}", dir_path.to_str().unwrap(),
                                                   why.description()),
            Ok(_) => ()
        }
    }

}


pub fn write_file(filepath: &str, content: &str) {
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

pub fn read_file(filename: &str) -> String {
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

