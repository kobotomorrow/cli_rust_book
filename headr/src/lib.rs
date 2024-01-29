use std::error::Error;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust head")]
pub struct Config {
    /// input file
    files: Vec<String>,
    
    /// TBD
    #[arg(short='n', long="lines")]
    lines: usize,

    /// TBD
    #[arg(short='c', long="bytes")]
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
