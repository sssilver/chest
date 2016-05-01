use std::error;
use std::fmt;


#[derive(Debug)]
pub enum ChestError {
    Generic(String)
}


impl fmt::Display for ChestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChestError::Generic(ref message) => write!(f, "Generic error: {}", message),
        }
    }
}


impl error::Error for ChestError {
    fn description(&self) -> &str {
        match *self {
            ChestError::Generic(ref message) => message
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            ChestError::Generic(ref message) => None,
        }
    }
}

