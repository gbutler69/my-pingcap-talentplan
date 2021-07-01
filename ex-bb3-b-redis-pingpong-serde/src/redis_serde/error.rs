#![cfg(test)]

use std::io;

use serde::{de, ser};
mod tests;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        todo!()
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        let message = io_error.to_string();
        Self {
            kind: ErrorKind::IoError(io_error),
            message,
        }
    }
}
