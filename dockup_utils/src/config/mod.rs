use std::path::PathBuf;

use super::files as files;
use super::ProgramDefinition;

pub const DOCKUP_CONFIG_FILENAME: &'static str = "dockup.yaml";

pub fn save_config_struct(config_dirpath: &str, programdef: &ProgramDefinition) -> String {
    info!("Save config file !");
    let yaml_str = ProgramDefinition::to_yaml(&programdef);

    //concatenate dirpath and default config filename
    let mut filepath = PathBuf::from(config_dirpath);
    filepath.push(DOCKUP_CONFIG_FILENAME);

    //write yaml to the dockup install file
    files::write_file(& filepath.to_str().unwrap(), &yaml_str);

    String::from(filepath.as_path().to_str().unwrap())

}

//Load the content of the filename given as a slice into a ProgramDefinition struct
pub fn load_config_struct(filename: &str) -> ProgramDefinition {
    let configfile_content = files::read_file(filename);

    ProgramDefinition::from_yaml(&configfile_content)

}