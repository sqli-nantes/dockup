use std::path::PathBuf;
use std::env;

use super::super::config as config;
use super::super::files as files;
use super::super::ProgramDefinition  as ProgramDefinition;

pub const DOCKUP_CONFIG_PATH: &'static str = ".dockup";


fn create_dockup_config_dir() -> String  {
    debug!("Try to create config dir");
    let mut absolute_dockup_path = PathBuf::new();

    match env::home_dir() {
        Some(path) => absolute_dockup_path.push(path),
        None => panic!("Failed to get your home dir!"),
    }
    absolute_dockup_path.push(DOCKUP_CONFIG_PATH);

    let dir_to_create = absolute_dockup_path.to_str().unwrap();

    files::create_dir(dir_to_create);

    debug!("Config dir is : {} ", dir_to_create);
    String::from(dir_to_create)
}

fn create_application_config_dir(programdef: &ProgramDefinition, dockup_config_dir: &str) -> String  {
    debug!("Try to create application config dir");

    //create the directory
    let mut absolute_application_dockup_path = PathBuf::from(dockup_config_dir);
    absolute_application_dockup_path.push(&programdef.name);

    let dir_to_create = absolute_application_dockup_path.to_str().unwrap();

    files::create_dir(dir_to_create);

    debug!("Application dir is : {} ", dir_to_create);
    String::from(dir_to_create)
}

pub fn execute(config_path: &str) {
    // First, create the dockup config dir
    let dockup_config_dir = create_dockup_config_dir();

    // Next, load the config file and create a directory inside the dockup config dir to store the application config yaml
    let programdef: ProgramDefinition = config::load_config_struct(config_path);
    info!("Loaded configuration : \n {}", programdef);

    // Then, copy the application config yaml to the config dir
    let application_config_dir = create_application_config_dir(&programdef, &dockup_config_dir);
    config::save_config_struct(&application_config_dir, &programdef);

    // Finally, create an alias for the execution of the command


    //TODO add to PATH?

}