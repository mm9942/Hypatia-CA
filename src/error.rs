use owo_colors::OwoColorize;
use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Rcgen(rcgen::Error),
    Serde(serde_json::Error),
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "{}", format!("IO error: {e}").red()),
            Error::Rcgen(e) => write!(f, "{}", format!("rcgen error: {e}").red()),
            Error::Serde(e) => write!(f, "{}", format!("serde error: {e}").red()),
            Error::Other(m) => write!(f, "{}", m.red()),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<rcgen::Error> for Error {
    fn from(e: rcgen::Error) -> Self {
        Error::Rcgen(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}
