#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramConfig {
    pub name: String,
    pub command: String,
    yaml: String,
}

