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

impl Error for ArgsError {}

#[derive(Debug)]
pub struct HttpError {
    status_code: u16,
    message: String
}

impl HttpError{
    pub fn new(status_code:u16, message:&str) -> HttpError {
        HttpError {
            status_code,
            message: message.to_string(),
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP Error {}: {}", self.status_code, self.message)
    }
}


// Implementa Error para ArgsError
impl Error for HttpError {}
