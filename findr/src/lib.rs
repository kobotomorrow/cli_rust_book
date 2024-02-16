use std::error::Error;

use clap::{Parser, ValueEnum};
use regex::Regex;

// use crate::EntryType::*;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone)]
enum EntryType {
    #[value(help="Directories (alias 'd')", alias="d")]
    Dir,

    #[value(help="Files (alias 'f')", alias="f")]
    File,

    #[value(help="Links (alias 'l')", alias="l")]
    Link,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust find")]
 pub struct Config {
    paths: Vec<String>,

    // num_argsについて
    // https://github.com/clap-rs/clap/issues/4655
    /// Name
    #[arg(short='n', long="name", num_args=0..)]
    names: Vec<Regex>,

    /// Entry type
    #[arg(short='t', long="type", num_args=0..)]
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let mut config = Config::parse();
    if config.paths.is_empty() {
        config.paths = vec![String::from(".")]
    }
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}