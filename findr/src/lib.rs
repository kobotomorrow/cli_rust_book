use std::error::Error;

use clap::{Parser, ValueEnum};
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

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
    let entry_types = config.entry_types;
    let names = config.names;
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => {
                    if !match_types(&entry, &entry_types) { continue; }
                    if !match_names(&entry, &names) { continue; }
                    println!("{}", entry.path().display())
                },
            }
        }
    }
    Ok(())
}

fn match_types(entry: &DirEntry, entry_types: &Vec<EntryType>) -> bool {
    if entry_types.is_empty() { return true }
    let file_type = entry.file_type();
    for entry_type in entry_types {
        let result = match entry_type {
            EntryType::Dir => { file_type.is_dir() },
            EntryType::File => { file_type.is_file() },
            EntryType::Link => { file_type.is_symlink() },
        };
        if result { return true }
    }
    false
}

fn match_names(entry: &DirEntry, names: &Vec<Regex>) -> bool {
    if names.is_empty() { return true };

    let file_name = if let Some(name) = entry.path().file_name() {
        name.to_str().unwrap()
    } else {
        ""
    };
    if file_name.is_empty() { return false };

    for name in names {
        if name.is_match(file_name) {
            return true
        };
    }
    false
}