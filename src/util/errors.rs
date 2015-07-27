use std::fmt;
use std::convert::From;
use std::io;
use std::error::Error as StdError;
use std::result;

use nix;

pub type Result<T> = result::Result<T, Error>;

/// An error arising from terminal operations.
///
/// The lower-level cause of the error, if any, will be returned by calling `cause()`.
#[derive(Debug)]
pub struct Error {
    err: Box<StdError + Send + Sync>,
}

impl Error {
    pub fn new<E>(error: E) -> Error
        where E: Into<Box<StdError + Send + Sync>>
    {
        let err = error.into();
        Error {
            err: err,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.err.description()
    }

    fn cause(&self) -> Option<&StdError> {
        self.err.cause()
    }
}

impl From<nix::Error> for Error {
    fn from(err: nix::Error) -> Self {
        Error::new(err.errno().desc())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::new(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.err.fmt(f)
    }
}

