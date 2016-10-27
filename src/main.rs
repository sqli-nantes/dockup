#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::App;
use clap::AppSettings;

extern crate dockup_utils;
use dockup_utils::config as config;
use dockup_utils::logger as logger;
use dockup_utils::system as system;
use dockup_utils::config::ProgramConfig as ProgramConfig;

use std::error::Error;


/// Each Dockup command will have an execute from a config path method
trait ExecutableCliCommand {
    fn execute(app_config_path: &str);
}

/// Representation of the Dockup install commmand
pub struct Install;

impl ExecutableCliCommand for Install {

    fn execute(app_config_path: &str) {
        // First, create the dockup config object
        let dockup_config = config::DockupConfig::new();


        // Next, load the config file and create a directory inside the dockup config dir to store the application config yaml
        let program_config = ProgramConfig::new(app_config_path);
        info! ("Loaded configuration : \n {}", program_config);


        //create a directory to store all the specific
        let program_config_dir = program_config.create_config_dir(&dockup_config.dockup_dir);


        // Then, copy the application config yaml to the config dir
        let config_path = program_config.save(&program_config_dir);

        // Finally, create an alias for the execution of the command
        system::WrappedDockupRun::new(&program_config_dir, &program_config.name, &config_path).as_callable_cli();

    }
}

/// Representation of the Dockup run commmand
pub struct Run;

impl ExecutableCliCommand for Run {

    fn execute(config_path: &str) {

        // Next, load the config file and create a directory inside the dockup config dir to store the application config yaml
        let program_config: ProgramConfig = ProgramConfig::new(config_path);
        info!("Loaded configuration : \n {}", program_config);

        system::execute_command(&program_config.command);
    }
}

fn main() {
    //Init the logger
    match logger::init() {
        Err(why) => panic!("couldn't init logger: {}, caused by {:?}",  why.description(), why.cause()),
        Ok(_) => println!("logger initialised"),
    }

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("resources/dockup_cli.yaml");
    let matches = App::from_yaml(yaml).setting(AppSettings::SubcommandRequired).get_matches();


    if let Some(matches) = matches.subcommand_matches("install") {

        let config_path = matches.value_of("config").unwrap_or(config::DOCKUP_CONFIG_FILENAME);

        Install::execute(config_path);

    } else if let Some(matches) = matches.subcommand_matches("run") {

        let config_path = matches.value_of("config").unwrap_or(config::DOCKUP_CONFIG_FILENAME);

        Run::execute(config_path);

    }
}