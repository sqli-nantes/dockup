use std::fmt;


pub struct ProgramDefinition {
    pub name: String,
    pub cmd: String,
    pub os: Vec<String>,
}

impl fmt::Display for ProgramDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Your program loaded  attributes :\n - name: {}\n - command : {}\n - targeted os: {:?}\n ", self.name, self.cmd, self.os)

    }
}