use std::{
    error::Error,
    io::{self},
    net, vec,
};

mod redis_serde;

fn main() -> Result<(), Box<dyn Error>> {
    let args = arguments();
    match args.subcommand() {
        ("server", Some(_)) => start_server(socket_addresses_from(&args)?),
        ("client", Some(_)) => start_client(socket_addresses_from(&args)?),
        _ => handle_invalid_command(),
    }
}

fn arguments() -> clap::ArgMatches<'static> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Ping-Pong Redis Protocol")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(clap::Arg::with_name("host")
                .short("H")
                .long("host")
                .takes_value(true)
                .required(false)
                .default_value("localhost"))
        .arg(clap::Arg::with_name("port")
                .short("P")
                .long("port")
                .takes_value(true)
                .required(false)
                .default_value("65000"))
        .subcommand(
            clap::App::new("server")
                .about("starts the server listening on the given <host> and <port>")
        )
        .subcommand(
            clap::App::new("client")
                .about("starts the client to connect to server at given <host> and <port>")
        )
        .after_help(
            "redis-pingpong is a client and server to send PING messages from the client to the server which responds \
             with a PONG message. All messages are using the Redis protocol. \
                It is implemented as part of the PingCAP Talent Plan tutorial series for Rust.",
        )
        .get_matches()
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
    Ok(redis_serde::handle_command(
        &mut io::BufReader::new(stream.try_clone()?),
        &mut io::BufWriter::new(stream),
    )?)
}

fn start_client(connect_to: vec::IntoIter<net::SocketAddr>) -> Result<(), Box<dyn Error>> {
    let stream = net::TcpStream::connect(connect_to.collect::<Vec<_>>().as_slice())?;
    Ok(redis_serde::send_ping_and_handle_response(
        &mut io::BufReader::new(stream.try_clone()?),
        &mut io::BufWriter::new(stream),
    )?)
}

fn handle_invalid_command() -> Result<(), Box<dyn Error>> {
    eprintln!("Invalid Options or Command");
    std::process::exit(1)
}
