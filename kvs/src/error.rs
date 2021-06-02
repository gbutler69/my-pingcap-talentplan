use failure::{Backtrace, Fail};
use std::{
    fmt::{self, Display},
    io,
};

/// kvs error type
#[derive(Debug)]
pub struct Error {
    inner: failure::Context<ErrorKind>,
}

impl Error {
    /// create a new kvs Error of the given ErrorKind
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            inner: failure::Context::new(kind),
        }
    }

    /// gets a referenced to the ErrorKind of this Error
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

/// kvs error kind
#[derive(Copy, Clone, Debug, PartialEq, Eq, failure_derive::Fail)]
pub enum ErrorKind {
    #[fail(display = "An I/O error occurred")]
    /// raised if there is an I/O error
    IoError,
    #[fail(display = "Key not present in database")]
    /// raised if key is not present on a remove
    KeyNotPresent,
    #[fail(display = "An unknown error occurred")]
    /// raised for any other error
    UnknownError,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Self {
            inner: failure::Context::new(ErrorKind::IoError),
        }
    }
}

/// kvs result type
pub type Result<T> = std::result::Result<T, Error>;
