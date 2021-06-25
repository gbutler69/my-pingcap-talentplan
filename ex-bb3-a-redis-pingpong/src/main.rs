use std::{
    error::Error,
    io::{self, Write},
    net, str, vec,
};

use clap::{App, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let args = arguments();
    match args.subcommand() {
        ("server", Some(_)) => start_server(socket_addresses_from(&args)?),
        ("client", Some(_)) => start_client(socket_addresses_from(&args)?),
        _ => handle_invalid_command(),
    }
}

fn arguments() -> clap::ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Ping-Pong Redis Protocol")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("host")
                .short("H")
                .long("host")
                .takes_value(true)
                .required(false)
                .default_value("localhost"))
        .arg(Arg::with_name("port")
                .short("P")
                .long("port")
                .takes_value(true)
                .required(false)
                .default_value("65000"))
        .subcommand(
            App::new("server")
                .about("starts the server listening on the given <host> and <port>")
        )
        .subcommand(
            App::new("client")
                .about("starts the client to connect to server at given <host> and <port>")
        )
        .after_help(
            "redis-pingpong is a client and server to send PING messages from the client to the server which responds \
             with a PONG message. All messages are using the Redis protocol. \
                It is implemented as part of the PingCAP Talent Plan tutorial series for Rust.",
        )
        .get_matches()
}

#[derive(Debug)]
enum FieldType {
    SimpleString {
        command: Option<String>,
        message: String,
    },
    Error {
        kind: Option<String>,
        message: String,
    },
    Integer(u64),
    BulkString(Vec<u8>),
    Array(Vec<FieldType>),
}

fn socket_addresses_from(args: &clap::ArgMatches) -> io::Result<vec::IntoIter<net::SocketAddr>> {
    net::ToSocketAddrs::to_socket_addrs(&(
        args.value_of("host")
            .expect("invalid IP address or host name"),
        args.value_of("port")
            .unwrap()
            .parse::<u16>()
            .expect("invalid port number - must be between 1 and 65535"),
    ))
}

fn start_server(listen_on: vec::IntoIter<net::SocketAddr>) -> Result<(), Box<dyn Error>> {
    let listener = net::TcpListener::bind(listen_on.collect::<Vec<_>>().as_slice())?;
    for maybe_stream in listener.incoming() {
        match maybe_stream {
            Ok(stream) => handle_connection(stream)?,
            Err(err) => return Err(Box::new(err)),
        }
    }
    Ok(())
}

fn handle_connection(stream: net::TcpStream) -> Result<(), Box<dyn Error>> {
    let read_stream = io::BufReader::new(stream.try_clone()?);
    let write_stream = io::BufWriter::new(stream);
    let _ = expect_simple_command(read_stream, "PING")?;
    println!("Received PING. Sending PONG!");
    send_simple_message(write_stream, "PONG")
}

fn start_client(connect_to: vec::IntoIter<net::SocketAddr>) -> Result<(), Box<dyn Error>> {
    let conn = net::TcpStream::connect(connect_to.collect::<Vec<_>>().as_slice())?;
    ping_server(conn)
}

fn ping_server(stream: net::TcpStream) -> Result<(), Box<dyn Error>> {
    let read_stream = io::BufReader::new(stream.try_clone()?);
    let write_stream = io::BufWriter::new(stream);
    let _ = send_simple_message(write_stream, "PING")?;
    println!("Sent PING.");
    let _ = expect_simple_command(read_stream, "PONG")?;
    println!("Received PONG.");
    Ok(())
}

fn handle_invalid_command() -> Result<(), Box<dyn Error>> {
    eprintln!("Invalid Options or Command");
    std::process::exit(1)
}

fn expect_simple_command(
    mut stream: io::BufReader<net::TcpStream>,
    expected_command: &str,
) -> Result<FieldType, Box<dyn Error>> {
    match read_protocol_message(&mut stream)? {
        FieldType::SimpleString {
            command: Some(command),
            message,
        } if command == expected_command => Ok(FieldType::SimpleString {
            command: Some(command),
            message,
        }),
        incorrect_message @ FieldType::SimpleString { .. }
        | incorrect_message @ FieldType::Error { .. }
        | incorrect_message @ FieldType::Integer(_)
        | incorrect_message @ FieldType::BulkString(_)
        | incorrect_message @ FieldType::Array(_) => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "I/O Error: Incorrect Message Type. Message received {:?}",
                incorrect_message
            ),
        ))),
    }
}

fn read_protocol_message(
    stream: &mut io::BufReader<net::TcpStream>,
) -> Result<FieldType, Box<dyn Error>> {
    let field_type_buf = &mut [u8::default()];
    io::Read::read_exact(stream, field_type_buf)?;
    #[allow(clippy::char_lit_as_u8)]
    match field_type_buf[0] {
        prefix if prefix == '+' as u8 => read_simple_string_from(stream),
        prefix if prefix == '-' as u8 => read_error_from(stream),
        prefix if prefix == ':' as u8 => read_integer_from(stream),
        prefix if prefix == '$' as u8 => read_bulk_data_from(stream),
        prefix if prefix == '*' as u8 => read_array_from(stream),
        unrecognized_prefix => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "I/O Error: Incorrect Field Prefix. Prefix received {}",
                unrecognized_prefix
            ),
        ))),
    }
}

fn read_simple_string_from(
    stream: &mut io::BufReader<net::TcpStream>,
) -> Result<FieldType, Box<dyn Error>> {
    let mut buf = String::default();
    let _ = io::BufRead::read_line(stream, &mut buf)?;
    buf.pop();
    if buf.chars().last().unwrap_or('\0') != '\r' {
        Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "I/O Error: Missing CR at end of Simple String. Found {}",
                buf.chars().last().unwrap_or('\0')
            ),
        )))
    } else {
        buf.pop();
        match buf.split_once(|c: char| c.is_whitespace()) {
            Some((command, message)) if command.chars().all(|c| c.is_uppercase()) => {
                Ok(FieldType::SimpleString {
                    command: Some(command.into()),
                    message: message.into(),
                })
            }
            None if buf.chars().all(|c| c.is_uppercase()) => Ok(FieldType::SimpleString {
                command: Some(buf),
                message: "".into(),
            }),
            Some(_) | None => Ok(FieldType::SimpleString {
                command: None,
                message: buf,
            }),
        }
    }
}

fn read_error_from(
    stream: &mut io::BufReader<net::TcpStream>,
) -> Result<FieldType, Box<dyn Error>> {
    todo!()
}

fn read_integer_from(
    stream: &mut io::BufReader<net::TcpStream>,
) -> Result<FieldType, Box<dyn Error>> {
    todo!()
}

fn read_bulk_data_from(
    stream: &mut io::BufReader<net::TcpStream>,
) -> Result<FieldType, Box<dyn Error>> {
    todo!()
}

fn read_array_from(
    stream: &mut io::BufReader<net::TcpStream>,
) -> Result<FieldType, Box<dyn Error>> {
    todo!()
}

fn send_simple_message(
    mut stream: io::BufWriter<net::TcpStream>,
    message: &str,
) -> Result<(), Box<dyn Error>> {
    let message = format!("+{}\r\n", message);
    let buf = message.as_bytes();
    let mut start = 0;
    loop {
        let written = stream.write(&buf[start..])?;
        if written + start == buf.len() {
            break;
        }
        start += written;
    }
    Ok(())
}
