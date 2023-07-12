#[derive(Debug)]
pub struct Commnads {
    pub command: String,
    pub description: String,
}

impl Commnads {
    pub fn new(c: String, d: String) -> Commnads {
        Commnads {
            command: c,
            description: d,
        }
    }
}

enum CommandsList {
    MKDIR,
}
