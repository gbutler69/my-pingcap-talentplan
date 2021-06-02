use std::path;

use clap::{App, Arg};
use kvs::Result;

fn main() -> Result<()> {
    match arguments().subcommand() {
        ("set", Some(args)) => handle_subcommand_set(args),
        ("get", Some(args)) => handle_subcommand_get(args),
        ("rm", Some(args)) => handle_subcommand_rm(args),
        _ => handle_invalid_command(),
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

fn handle_subcommand_set(args: &clap::ArgMatches) -> Result<()> {
    let store = &mut kvs::KvStore::<String, String>::open(path::Path::new("./"))?;
    store.set(
        args.value_of("key").unwrap().into(),
        args.value_of("value").unwrap().into(),
    )
}

fn handle_subcommand_get(args: &clap::ArgMatches) -> Result<()> {
    let store = &mut kvs::KvStore::<String, String>::open(path::Path::new("./"))?;
    match store.get(args.value_of("key").unwrap().into()) {
        Ok(Some(value)) => println!("{}", value),
        Ok(None) => println!("Key not found"),
        Err(err) => return Err(err),
    }
    Ok(())
}

fn handle_subcommand_rm(args: &clap::ArgMatches) -> Result<()> {
    let store = &mut kvs::KvStore::<String, String>::open(path::Path::new("./"))?;
    match store.remove(args.value_of("key").unwrap().into()) {
        Ok(_) => Ok(()),
        Err(err) if *err.kind() == kvs::ErrorKind::KeyNotPresent => {
            println!("Key not found");
            Err(err)
        }
        Err(err) => Err(err),
    }
}

fn handle_invalid_command() -> Result<()> {
    eprintln!("Invalid Options or Command");
    std::process::exit(1)
}
