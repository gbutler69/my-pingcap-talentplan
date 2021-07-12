#[cfg(test)]
mod tests;

mod error;

mod de;
mod ser;

pub use de::from_reader;
pub use ser::to_writer;

pub use error::{Error, ErrorKind, Result};

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Command {
    Ping,
    Pong,
}

pub(crate) fn handle_command(
    read_stream: std::io::BufReader<std::net::TcpStream>,
    write_stream: std::io::BufWriter<std::net::TcpStream>,
) -> Result<()> {
    todo!()
}

pub(crate) fn send_ping_and_handle_response(
    read_stream: std::io::BufReader<std::net::TcpStream>,
    write_stream: std::io::BufWriter<std::net::TcpStream>,
) -> Result<()> {
    todo!()
}
