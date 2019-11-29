use std::error::Error;
use std::fmt;

pub type ILertResult<T> = Result<T, ILertError>;

#[derive(Debug)]
pub struct ILertError {
    message: String
}

impl ILertError {
    pub fn new(message: &str) -> ILertError {
        ILertError {
            message: message.to_string()
        }
    }
}

impl fmt::Display for ILertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ILertError {
    fn description(&self) -> &str {
        &self.message
    }
}
