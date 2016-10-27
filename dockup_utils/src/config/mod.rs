#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

#[cfg(feature = "serde_macros")]
include!("config/config_type.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/config_type.rs"));

extern crate serde;
extern crate serde_yaml;

use std::env;
use super::file;
use std::path::PathBuf;
use std::path::Path;
use std::fmt;

pub const DOCKUP_CONFIG_FILENAME: &'static str = "dockup.yaml";
const DOCKUP_CONFIG_PATH: &'static str = ".dockup";

/// Configuration of Dockup : root directory, system configuration ...

pub struct DockupConfig  {
    pub dockup_dir: String,
}

impl DockupConfig {

    pub fn new() -> DockupConfig  {
        debug!("Try to create config dir");
        let mut config_path = PathBuf::new();

        match env::home_dir() {
            Some(path) => config_path.push(path),
            None => panic!("Failed to get your home dir!"),
        }
        config_path.push(DOCKUP_CONFIG_PATH);

        let config_path_as_str=config_path.to_str().unwrap();
        file::create_dir(config_path_as_str);

        debug!("Config dir is : {} ", config_path_as_str);

        DockupConfig { dockup_dir:  String::from(config_path_as_str) }
    }

}

/// Implementations for the Program Configuration

impl fmt::Display for ProgramConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your program loaded  attributes :\n - name: {}\n - command : {}\n ", self.name, self.command)

    }
}

impl ProgramConfig {

    pub fn new(app_config_path: &str) -> ProgramConfig {
        let yaml_str = file::read_file(app_config_path);
        serde_yaml::from_str(yaml_str.as_str()).unwrap()
    }

    pub fn create_config_dir(&self, dockup_dir : &str) -> String {
        let file_path = Path::new(dockup_dir).join(&self.name);

        file::create_dir(file_path.to_str().unwrap());

        String::from(file_path.to_str().unwrap())


    }

    pub fn save(&self, config_dirpath: &str) -> String {
        info!("Save application dockup config to config dir {}", config_dirpath);

        //concatenate dirpath and default config filename
        let file_path = Path::new(config_dirpath).join(DOCKUP_CONFIG_FILENAME);

        let yaml_str = serde_yaml::to_string(self).unwrap();

        //write yaml to the dockup install file
        file::write_file(&file_path.to_str().unwrap(), &yaml_str);

        String::from(file_path.as_path().to_str().unwrap())
    }
}