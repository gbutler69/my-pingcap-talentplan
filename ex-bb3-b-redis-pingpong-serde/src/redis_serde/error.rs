#![cfg(test)]

use std::{io, num, string};

use serde::{de, ser};
mod tests;

#[derive(Debug)]
pub enum ErrorKind {
    IoError(io::Error),
    ParseIntError(num::ParseIntError),
    ParseFloatError(num::ParseFloatError),
    FromUtf8Error(string::FromUtf8Error),
    DataError,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
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

impl From<num::ParseIntError> for Error {
    fn from(parse_error: num::ParseIntError) -> Self {
        let message = parse_error.to_string();
        Self {
            kind: ErrorKind::ParseIntError(parse_error),
            message,
        }
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(parse_error: num::ParseFloatError) -> Self {
        let message = parse_error.to_string();
        Self {
            kind: ErrorKind::ParseFloatError(parse_error),
            message,
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(parse_error: string::FromUtf8Error) -> Self {
        let message = parse_error.to_string();
        Self {
            kind: ErrorKind::FromUtf8Error(parse_error),
            message,
        }
    }
}
