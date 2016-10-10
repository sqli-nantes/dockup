use std::process::Command;
use std::path::PathBuf;

pub const SYS_CHMOD_CMD: &'static str = "chmod";
pub const MOD_X_FORALL: &'static str = "755";
pub const SUDO_BIN: &'static str = "sudo";
pub const LN_BIN: &'static str = "ln";
pub const SYM_ARG: &'static str = "-s";
pub const PATH_USR_BIN: &'static str = "/usr/bin";

pub fn make_executable(file_path: &str) {

    let command_chunks = [MOD_X_FORALL, file_path];

    let child = Command::new(SYS_CHMOD_CMD).args(&command_chunks).spawn()
        .expect(format!("failed to execute chmod 755 on {}", file_path).as_str());

    child.wait_with_output()
        .expect("failed to wait on child");
}

pub fn create_symlink_binary(source_cmd_path: &str, binary_name: &str) {

    let mut target_linkpath = PathBuf::from(PATH_USR_BIN);
    target_linkpath.push(binary_name);

    let command_chunks = [LN_BIN, SYM_ARG, source_cmd_path, target_linkpath.to_str().unwrap()];

    let child = Command::new(SUDO_BIN).args(&command_chunks).spawn()
        .expect(format!("failed to create symlink {} on {}", target_linkpath.to_str().unwrap(), source_cmd_path).as_str());

    child.wait_with_output()
        .expect("failed to wait on child");

}
