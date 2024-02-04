use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust wc")]
pub struct Config {
    files: Vec<String>,

    /// Show line count
    #[arg(short='l', long="lines")]
    lines: bool,

    /// Show word count
    #[arg(short='w', long="words")]
    words: bool,

    /// Show byte count
    #[arg(short='c', long="bytes", group = "show_options")]
    bytes: bool,

    /// Show character count
    #[arg(short='m', long="chars", group = "show_options")]
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let mut config = Config::parse();
    if config.files.is_empty() {
        config.files = vec![String::from("-")]
    };
    if [config.lines, config.words, config.bytes, config.chars].iter().all(|&x| !x){
        config.lines = true;
        config.words = true;
        config.bytes = true;
    };
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
