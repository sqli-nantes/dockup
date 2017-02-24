use std::error::Error;
use std::path::Path;
use std::fs::DirBuilder;
use std::io::prelude::*;
use std::fs::File;

pub fn create_dir(dir_path: &str) {
    let path = Path::new(dir_path);

    if !path.exists() {
        info!("Create the directory {}", path.display());
        match DirBuilder::new().recursive(true).create(path.to_str().unwrap()) {
            Err(why) => {
                panic!("couldn't create the directory {}: {}",
                       path.to_str().unwrap(),
                       why.description())
            }
            Ok(_) => (),
        }
    }

}

pub fn write_file(file_path: &str, content: &str) {
    let path = Path::new(file_path);

    // Create the file, returns `io::Result<File>`
    let mut file = match File::create(path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => {
            panic!("couldn't create the file {}: {}",
                   path.display(),
                   why.description())
        }
        Ok(file) => file,
    };

    // Write the content string to `file`, returns `io::Result<()>`
    match file.write_all(content.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}",
                   path.display(),
                   why.description())
        }
        Ok(_) => debug!("successfully wrote to {}", path.display()),
    }
}

pub fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&file_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => {
            panic!("couldn't open the file {}: {}",
                   path.display(),
                   why.description())
        }
        Ok(file) => file,
    };


    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why.description()),
        Ok(_) => debug!("{} contains:\n{}", path.display(), s),
    }

    s
}
