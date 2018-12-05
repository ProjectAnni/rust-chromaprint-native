use std::error::Error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, ChromaprintError>;

#[derive(Debug)]
pub struct ChromaprintError;

impl Error for ChromaprintError {}

impl fmt::Display for ChromaprintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occurred")
    }
}

pub fn handle_return(return_value: i32) -> Result<()> {
    if return_value == 0 {
        return Err(ChromaprintError);
    } else {
        return Ok(());
    }
}
