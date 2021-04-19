use failure::{Backtrace, Fail};
use std::fmt::{self, Display};

/// kvs error type
#[derive(Debug)]
pub struct Error {
    inner: failure::Context<ErrorKind>,
}

/// kvs error kind
#[derive(Copy, Clone, Debug, PartialEq, Eq, failure_derive::Fail)]
pub enum ErrorKind {
    #[fail(display = "A filesystem error occurred")]
    /// raised if there is an error accessing the filesystem
    FilesystemError,
    #[fail(display = "A network error occurred")]
    /// raised if there is a network error
    NetworkError,
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

/// kvs result type
pub type Result<T> = std::result::Result<T, Error>;
