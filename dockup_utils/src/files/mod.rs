use std::path::Path;
use std::fs::DirBuilder;

pub fn create_dir(filepath: &str)  {
    debug!("Try to create config dir");
    let dir_path = Path::new(filepath);


    //TODO deal with not a directory but exists
    if !dir_path.exists() {
        info!("Create the directory {}", dir_path.display());
        DirBuilder::new().recursive(true).create(dir_path.to_str().unwrap());
    }

}