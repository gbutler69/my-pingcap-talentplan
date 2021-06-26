#![cfg(test)]
mod tests;

pub enum ErrorKind {}

pub struct Error {
    kind: ErrorKind,
    message: String,
}

pub type Result<T> = std::result::Result<T, Error>;
