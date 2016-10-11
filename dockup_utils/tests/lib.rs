extern crate dockup_utils;

use dockup_utils::config as config;
use dockup_utils::ProgramDefinition;

#[test]
fn configuration_loaded(){

    let filename = String::from("tests/resources/dockup-config.yaml");
    let program_definition = config::load_config_struct(&filename);

    assert_eq!(program_definition.command,"docker run --rm rguillom/reveal");
    assert_eq!(program_definition.name,"unit-application-test");
}

#[test]
fn save_configuration(){
    let filepath = String::from("/tmp");
    let program_definition = ProgramDefinition {
        name: String::from("myapp"),
        command: String::from("execute --with args"),
    };


    config::save_config_struct(&filepath, &program_definition);
}
