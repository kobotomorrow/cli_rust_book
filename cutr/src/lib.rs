use std::{error::Error, ops::Range};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust find")]
struct Input {
    files: Vec<String>,

    /// Field delimiter [default:  ]
    #[arg(short='d', long="delim", default_value = " ")]
    delimiter: char,
    
    /// Selected bytes
    #[arg(short='b', long="bytes", group = "selected_options")]
    bytes: Option<String>,

    /// Selected chars
    #[arg(short='c', long="chars", group = "selected_options")]
    chars: Option<String>,

    /// Selected fields
    #[arg(short='f', long="fields", group = "selected_options")]
    fields: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    extract: Extract,
    delimiter: u8,
}

pub fn get_args() -> MyResult<Config> {
    let input = Input::parse();
    let files = if input.files.is_empty() { vec![String::from("-")] } else { input.files };
    // TODO: convert PositionList
    // TODO: convert delim
    Ok(Config{
        files: files,
        extract: Extract::Bytes(vec![1..3]),
        delimiter: 1,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
