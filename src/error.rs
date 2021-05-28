//! Error and result types which are passed by the library.

use reqwest::Error as HTTPError;
use std::error::Error as StdError;
use std::fmt::Display;
use std::result::Result as StdResult;

/// The base Result type which is used in the library.
pub type Result<T> = StdResult<T, Error>;

/// An error which is returned from the library.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An error occured while interacting with the AT API.
    Request(Box<HTTPError>),
}

impl From<HTTPError> for Error {
    fn from(e: HTTPError) -> Self {
        Self::Request(Box::new(e))
    }
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::Request(e) => write!(f, "HTTP request error: {}", e),
        }
    }
}
