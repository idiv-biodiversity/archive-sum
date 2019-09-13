use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorKind {
    Io,
    LibArchive,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Error {
    pub message: String,
    pub kind: ErrorKind,
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {
            message: String::from(message),
            kind: ErrorKind::LibArchive,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            message: format!("{}", error),
            kind: ErrorKind::Io,
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &*self.message
    }
}
