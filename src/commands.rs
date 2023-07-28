#[derive(Debug)]
pub struct Commnads;

impl Commnads {
    pub fn get_command(command: Vec<String>) -> CommandsList {
        CommandsList::MKNODE
    }
}

pub enum CommandsList {
    MKNODE,
    RM,
}
