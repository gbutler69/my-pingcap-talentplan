#[cfg(test)]
mod tests;

mod error;

mod de;
mod ser;

use std::io;

pub use de::from_reader;
pub use ser::to_writer;

pub use error::{Error, ErrorKind, Result};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Command {
    Ping,
    Pong,
}

pub fn handle_command<R: io::Read, W: io::Write>(
    reader: &mut io::BufReader<R>,
    writer: &mut io::BufWriter<W>,
) -> Result<()> {
    match from_reader::<_, Command>(reader)? {
        Command::Ping => {
            println!("Ping Received.");
            to_writer(writer, Command::Pong)?;
            println!("Pong Sent!");
            Ok(())
        }
        _ => Err(Error {
            kind: ErrorKind::DataError,
            message: "Expected a Ping Command. Received something else.".into(),
        }),
    }
}

pub fn send_ping_and_handle_response<R: io::Read, W: io::Write>(
    reader: &mut io::BufReader<R>,
    writer: &mut io::BufWriter<W>,
) -> Result<()> {
    to_writer(writer, Command::Ping)?;
    println!("Ping Sent.");
    match from_reader::<_, Command>(reader)? {
        Command::Pong => {
            println!("Pong Received!");
            Ok(())
        }
        _ => Err(Error {
            kind: ErrorKind::DataError,
            message: "Expected a Pong Response. Received something else.".into(),
        }),
    }
}
