use std::fmt;
use std::error::Error;

use nix;

#[derive(Debug)]
pub struct TtyError {
    description: &'static str,
}

impl TtyError {
    pub fn new(desc: &'static str) -> TtyError {
        TtyError {
            description: desc,
        }
    }

    pub fn from_nix(e: nix::Error) -> TtyError {
        TtyError {
            description: e.errno().desc(),
        }
    }
}

impl Error for TtyError {
    fn description(&self) -> &str {
        self.description
    }
}

impl fmt::Display for TtyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

