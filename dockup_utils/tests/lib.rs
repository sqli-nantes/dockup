extern crate dockup_utils;

use dockup_utils::config::load;

#[test]
fn configuration_loaded(){

    let filename = String::from("tests/resources/dockup-config.yaml");
    let program_definition = load::load_config_struct(&filename);

    assert_eq!(program_definition.cmd,"docker run --rm rguillom/reveal");
    assert_eq!(program_definition.name,"unit-application-test");
    assert_eq!(program_definition.os,["linux","windows"]);
}
