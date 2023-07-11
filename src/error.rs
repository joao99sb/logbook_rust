use std::io::Write;
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct StatefulListStateInfoError;

impl fmt::Display for StatefulListStateInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Error in Statefullist")
    }
}
impl Error for StatefulListStateInfoError {}
