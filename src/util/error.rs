//! Errors.

use std::fmt;
use std::convert::From;
use std::io;
use std::error::Error as StdError;

use nix;

#[derive(Debug)]
pub struct Error {
    description: &'static str,
    cause: Option<Box<StdError>>,
}

impl Error {
    pub fn new(desc: &'static str) -> Error {
        Error {
            description: desc,
            cause: None,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.description
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Error::new(err.errno().desc())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            description: "internal io error",
            cause: Some(Box::new(err)),
        }
    }
}

impl From<io::CharsError> for Error {
    fn from(err: io::CharsError) -> Self {
        Error {
            description: "utf8 translation error",
            cause: Some(Box::new(err)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

