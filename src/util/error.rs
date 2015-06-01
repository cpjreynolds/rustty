use std::fmt;
use std::convert::From;
use std::io;
use std::error::Error as StdError;

use nix;
use core::chars::CharStreamError;

/// An error arising from terminal operations.
///
/// The lower-level cause of the error, if any, will be returned by calling `cause()`.
///
/// **Note:** Errors arising from system calls will return `None` when calling `cause()` as `nix`
/// errors do not implement `Error`. In this case, `description()` will return the standard `errno`
/// description.
#[derive(Debug)]
pub struct Error {
    description: &'static str,
    cause: Option<Box<StdError>>,
}

impl Error {
    /// Creates a new `Error` with the given description.
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

    fn cause(&self) -> Option<&StdError> {
        if let Some(ref err) = self.cause {
            Some(&**err)
        } else {
            None
        }
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Error {
            description: err.errno().desc(),
            cause: None,
        }
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

impl From<CharStreamError> for Error {
    fn from(err: CharStreamError) -> Self {
        Error {
            description: "utf8 encoding error",
            cause: Some(Box::new(err)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", StdError::description(self))
    }
}

