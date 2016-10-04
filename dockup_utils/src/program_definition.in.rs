extern crate serde;
extern crate serde_yaml;

use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProgramDefinition {
    pub name: String,
    pub command: String,
    pub os: Vec<String>,
}

impl fmt::Display for ProgramDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your program loaded  attributes :\n - name: {}\n - command : {}\n - targeted os: {:?}\n ", self.name, self.command, self.os)

    }
}

impl ProgramDefinition {
    pub fn from_yaml(s: &String) -> ProgramDefinition {
        serde_yaml::from_str(&s).unwrap()
    }

    pub fn to_yaml(p: &ProgramDefinition) -> String{
        serde_yaml::to_string(p).unwrap()
    }
 }