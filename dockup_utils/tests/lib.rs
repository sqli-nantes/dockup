extern crate dockup_utils;

use dockup_utils::config as config;
use dockup_utils::config::ProgramConfig;

#[test]
fn configuration_loaded_and_saved() {
    let filename = String::from("tests/resources/dockup-config.yaml");

    let program_config = ProgramConfig::new(filename.as_ref());

    assert_eq!(program_config.command, "docker run --rm rguillom/reveal");
    assert_eq!(program_config.name, "unit-application-test");

    let config_app_dir: String = program_config.create_config_dir("/tmp");
    assert_eq!(config_app_dir, "/tmp/unit-application-test");

    let saved_filename: String = program_config.save(config_app_dir.as_ref());
    let mut expected = String::from("/tmp/unit-application-test/");
    expected.push_str(config::DOCKUP_CONFIG_FILENAME);
    assert_eq!(saved_filename, expected);

}
