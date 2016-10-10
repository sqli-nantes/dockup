use super::super::config as config;
use super::super::ProgramDefinition  as ProgramDefinition;


pub fn execute(config_path: &str) {

    // Next, load the config file and create a directory inside the dockup config dir to store the application config yaml
    let programdef: ProgramDefinition = config::load_config_struct(config_path);
    info!("Loaded configuration : \n {}", programdef);

    let child = super::create_command_from_programdef(programdef).spawn()
        .expect("failed to execute process");


    child.wait_with_output()
        .expect("failed to wait on child");

}