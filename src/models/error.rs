use std::{error::Error, fmt};



#[derive(Debug)]
pub struct ArgsError {
    message: String,
}

impl ArgsError {
    pub fn new(msg: &str) -> ArgsError {
        ArgsError{
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for ArgsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implementa Error para ArgsError
impl Error for ArgsError {}