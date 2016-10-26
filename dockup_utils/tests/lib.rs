extern crate dockup_utils;

use dockup_utils::config as config;
use dockup_utils::ApplicationDockupConfig;

#[test]
fn configuration_loaded(){

    let filename = String::from("tests/resources/dockup-config.yaml");

    let configfile_content = files::read_file(filename);

    let applicationConfig = ApplicationDockupConfig::new(&configfile_content);

    assert_eq!(applicationConfig.command,"docker run --rm rguillom/reveal");
    assert_eq!(applicationConfig.name,"unit-application-test");
}

#[test]
fn save_configuration(){
    let filepath = String::from("/tmp");
    let applicationConfig = ApplicationDockupConfig::new("name: myapp \\ncommand: execute --with args");

    let saved_filename = applicationConfig.save();
    assert_eq!(applicationConfig.command,"execute --with args");
}
