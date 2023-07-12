use std::{error::Error, fmt};


pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct StatefulListStateInfoError;

impl fmt::Display for StatefulListStateInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Error in Statefullist")
    }
}
impl Error for StatefulListStateInfoError {}
