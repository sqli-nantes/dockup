extern crate dockup_utils;

use dockup_utils::config;
use dockup_utils::ProgramDefinition;

#[test]
fn configuration_loaded(){

    let filename = String::from("tests/resources/dockup-config.yaml");
    let program_definition = config::load_config_struct(&filename);

    assert_eq!(program_definition.command,"docker run --rm rguillom/reveal");
    assert_eq!(program_definition.name,"unit-application-test");
    assert_eq!(program_definition.os,["linux","windows"]);
}

#[test]
fn save_configuration(){
    let filepath = String::from("target/dockup-test.yaml");
    let program_definition = ProgramDefinition {
        name: String::from("myapp"),
        command: String::from("execute --with args"),
        os: Vec::new()
    };


    config::save_config_struc(&filepath, &program_definition);
}
