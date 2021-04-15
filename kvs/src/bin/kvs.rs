use clap::{App, Arg};

fn main() {
    match arguments().subcommand() {
        ("set", Some(args)) => handle_subcommand_set(args),
        ("get", Some(args)) => handle_subcommand_get(args),
        ("rm", Some(args)) => handle_subcommand_rm(args),
        _ => std::process::exit(1),
    }
}

fn arguments() -> clap::ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Key-Value Store")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            App::new("set")
                .about("sets a <key> to the given <value>")
                .arg(Arg::with_name("key").index(1).required(true))
                .arg(Arg::with_name("value").index(2).required(true)),
        )
        .subcommand(
            App::new("get")
                .about("given a <key> gets the given <value> (if present)")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("remove the given <key> (and associated value) if present")
                .arg(Arg::with_name("key").index(1).required(true)),
        )
        .after_help(
            "kvs is a command-line program to act as a key-value store. \
                It is implemented as part of the PingCAP Talent Plan tutorial series for Rust.",
        )
        .get_matches()
}

fn handle_subcommand_set(args: &clap::ArgMatches) {
    eprintln!("unimplemented");
    std::process::exit(2);
}

fn handle_subcommand_get(args: &clap::ArgMatches) {
    eprintln!("unimplemented");
    std::process::exit(3);
}

fn handle_subcommand_rm(args: &clap::ArgMatches) {
    eprintln!("unimplemented");
    std::process::exit(4);
}
