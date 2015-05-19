use std::fmt;
use std::convert::From;
use std::io;
use std::error::Error;

use nix;

#[derive(Debug)]
pub struct TtyError {
    description: &'static str,
    cause: Option<Box<Error>>,
}

impl TtyError {
    pub fn new(desc: &'static str) -> TtyError {
        TtyError {
            description: desc,
            cause: None,
        }
    }

    pub fn from_nix(e: nix::Error) -> TtyError {
            TtyError::new(e.errno().desc())
    }
}

impl Error for TtyError {
    fn description(&self) -> &str {
        self.description
    }
}

impl From<nix::Error> for TtyError {
    fn from(err: nix::Error) -> Self {
        TtyError::new(err.errno().desc())
    }
}

impl From<io::Error> for TtyError {
    fn from(err: io::Error) -> Self {
        TtyError {
            description: "internal io error",
            cause: Some(Box::new(err)),
        }
    }
}

impl fmt::Display for TtyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

