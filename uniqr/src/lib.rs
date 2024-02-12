use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust uniq")]
pub struct Config {
    in_file: Option<String>,

    out_file: Option<String>,

    /// Show counts
    #[arg(short='c', long="count")]
    count: bool,
}

pub fn get_args() -> MyResult<Config> {
    let mut config = Config::parse();
    if config.in_file.is_none() {
        config.in_file = Some(String::from("-"))
    }
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    let input_filename = config.in_file.unwrap();
    let mut file = open(&input_filename).map_err(|e|
        format!("{}: {}", input_filename, e)
    )?;
    let mut line = String::new();
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
