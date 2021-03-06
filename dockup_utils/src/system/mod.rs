use std::process::*;
use std::path::PathBuf;
use std::io::Cursor;

extern crate rustache;
use self::rustache::{Render, HashBuilder};

use super::file;

const SYS_CHMOD_CMD: &'static str = "chmod";
const MOD_X_FORALL: &'static str = "755";
const SUDO_BIN: &'static str = "sudo";
const LN_BIN: &'static str = "ln";
const SYM_ARG: &'static str = "-s";
const PATH_USR_LOCAL_BIN: &'static str = "/usr/local/bin";
const KEY_CONFIG: &'static str = "config_path";


/// Encapsulation of a dockup run command
///
/// When a path command is called, the corresponding dockup run + config will be executed. This execution is wrapped in a shell in order for further maintainability
/// The shell is editable in a template in resources/exe_dockup.tmpl
/// This structure and its methods will provide all the elements (path, docker comment, etc.) to build and execute the wrapping Shell
///
pub struct WrappedDockupRun {
    pub dest_command_file: String,
    config_path: String,
    command: String,
}


impl WrappedDockupRun {
    pub fn new(app_config_path: &str,
               command_name: &str,
               to_config_path: &str)
               -> WrappedDockupRun {
        let mut dest_path = PathBuf::from(app_config_path);
        dest_path.push(command_name);

        WrappedDockupRun {
            dest_command_file: String::from(dest_path.as_path().to_str().unwrap()),
            config_path: String::from(to_config_path),
            command: String::from(command_name),
        }
    }

    pub fn as_callable_cli(&self) {
        let encapsulated_content = self.encapsulate_run_command();
        debug!("Encapsuled content to run is {}", encapsulated_content);

        file::write_file(&self.dest_command_file, encapsulated_content.as_str());

        make_executable(&self.dest_command_file);
        create_symlink_binary(&self.dest_command_file, &self.command);
    }

    fn encapsulate_run_command(&self) -> String {
        let template_str = include_str!("resources/exe_dockup.tmpl");
        let data = HashBuilder::new().insert(KEY_CONFIG, self.config_path.as_str());
        let mut rv = Cursor::new(Vec::new());

        data.render(template_str, &mut rv).unwrap();

        String::from_utf8(rv.into_inner()).unwrap()
    }
}

// TODO : Set the system functions as a 'view of the current system'

/// System utilities
/// make a file runnable (+x)
pub fn make_executable(file_path: &str) {

    let command_chunks = [MOD_X_FORALL, file_path];

    let child = Command::new(SYS_CHMOD_CMD)
        .args(&command_chunks)
        .spawn()
        .expect(format!("failed to execute chmod 755 on {}", file_path).as_str());

    child.wait_with_output()
        .expect("failed to wait on child");
}

/// create a symbolic link to a file in the usr local bin directory of the host
pub fn create_symlink_binary(file_path: &str, binary_name: &str) {
    let mut target_linkpath = PathBuf::from(PATH_USR_LOCAL_BIN);
    target_linkpath.push(binary_name);

    let command_chunks = [LN_BIN, SYM_ARG, file_path, target_linkpath.to_str().unwrap()];

    let child = Command::new(SUDO_BIN)
        .args(&command_chunks)
        .spawn()
        .expect(format!("failed to create symlink {} on {}",
                        target_linkpath.to_str().unwrap(),
                        file_path)
            .as_str());

    child.wait_with_output()
        .expect("failed to wait on child");
}

/// Execute command from a str
pub fn execute_command(command_str: &str) {
    debug!("Execute the command : {}", command_str);

    // Make a vec of all arguments included the command binary
    let command_chunks: Vec<&str> = command_str.split_whitespace().collect();

    // Separate the command name from the args
    let command_name_args = command_chunks.split_first();
    debug!("Execute the command : {}", command_name_args.unwrap().0);
    Command::new(command_name_args.unwrap().0)
        .args(command_name_args.unwrap().1)
        .output()
        .expect("failed to execute process");

}

// Execute command from a str but there is no panic if command is wrong
pub fn execute_command_without_panic(command_str: &str) -> bool {
    debug!("Execute the command : {}", command_str);

    // Make a vec of all arguments included the command binary
    let command_chunks: Vec<&str> = command_str.split_whitespace().collect();

    // Separate the command name from the args
    let command_name_args = command_chunks.split_first();

    Command::new(command_name_args.unwrap().0)
        .args(command_name_args.unwrap().1)
        .stdout(Stdio::piped())
        .output()
        .is_ok()
}
