#[cfg(test)]
mod tests;

mod error;

mod de;
mod ser;

pub use de::from_reader;
pub use ser::to_writer;

pub use error::{Error, ErrorKind, Result};
