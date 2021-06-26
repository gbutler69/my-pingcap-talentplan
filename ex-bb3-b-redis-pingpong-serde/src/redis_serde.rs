#![cfg(test)]
mod error;
mod tests;

use serde::{ser, Serialize};

use error::{Error, ErrorKind, Result};
