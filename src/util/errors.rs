use std::fmt;
use std::convert::From;
use std::io;
use std::error::Error as StdError;
use std::result;

use term::terminfo;
use nix;

pub type Result<T> = result::Result<T, Error>;

/// An error arising from terminal operations.
///
/// The lower-level cause of the error, if any, will be returned by calling `cause()`.
///
/// **Note:** Errors arising from system calls will return `None` when calling `cause()` as `nix`
/// errors do not implement `Error`. In this case, `description()` will return the standard `errno`
/// description.
#[derive(Debug)]
pub struct Error {
    desc: String,
    cause: Option<Box<StdError>>,
}

impl Error {
    pub fn new(desc: &str) -> Error {
        Error {
            desc: String::from(desc),
            cause: None,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.desc[..]
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
            desc: String::from(err.errno().desc()),
            cause: None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            desc: String::from(err.description()),
            cause: Some(Box::new(err)),
        }
    }
}

impl From<terminfo::Error> for Error {
    fn from(err: terminfo::Error) -> Self {
        Error {
            desc: String::from(err.description()),
            cause: Some(Box::new(err)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc)
    }
}

