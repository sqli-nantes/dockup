extern crate serde_json;

extern crate curl;
extern crate rustc_serialize;

use self::rustc_serialize::base64::FromBase64;
use self::curl::easy::{Easy, List};
use super::file;
use super::system;

const DOCKER_OFFICIAL_INSTALL_SH: &'static str = "https://api.github.\
                                                  com/repos/docker/docker/contents/hack/install.sh";

const DOCKER_TEST_SCRIPT: &'static str = "docker version";

const DOCKER_INSTALL_SCRIPT_PATH: &'static str = "/tmp/docker_install.sh";

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubAPIResult {
    pub content: String,
}

pub fn ensure_docker_locally_available() {

    if !is_docker_installed() {
        info!("Docker is not present and will be installed first");
        execute_docker_install();
    }
}

fn is_docker_installed() -> bool {
    debug!("docker test on : {}", DOCKER_TEST_SCRIPT);
    system::execute_command_without_panic(DOCKER_TEST_SCRIPT)
}



fn execute_docker_install() {
    // First step : download a Json containing the script content from the githubAPI
    let api_result = download_script_content_with_github_api();

    // Next decode the resulting base64 content
    let decoded_content = String::from_utf8(api_result.content.from_base64().unwrap()).unwrap();
    debug!("Decoded GitHub API Content for \"{}\" request : {}",
           DOCKER_OFFICIAL_INSTALL_SH,
           decoded_content);

    // write the install script in a file
    file::write_file(DOCKER_INSTALL_SCRIPT_PATH, &decoded_content.as_str());

    // next make executable and execute docker install
    system::make_executable(DOCKER_INSTALL_SCRIPT_PATH);
    let command: String = format!("{} {}", "sh", DOCKER_INSTALL_SCRIPT_PATH);
    system::execute_command(command.as_str());
}

fn download_script_content_with_github_api() -> GithubAPIResult {

    let mut easy = Easy::new();
    easy.url(DOCKER_OFFICIAL_INSTALL_SH).unwrap();

    let mut list = List::new();
    list.append("Accept: application/vnd.github.v3+json").unwrap();
    list.append("User-Agent: DockupApp").unwrap();
    easy.http_headers(list).unwrap();

    match easy.fail_on_error(true) {
        Err(why) => panic!("Error occured : {}", why),
        Ok(_) => debug!("Configured to fail on error"),
    }

    let mut content = Vec::new();

    {
        let mut transfer = easy.transfer();

        transfer.write_function(|data| {

                content.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }

    let content_decoded_str = String::from_utf8(content).unwrap();
    let api_result: GithubAPIResult = serde_json::from_str(content_decoded_str.as_str()).unwrap();

    api_result

}
