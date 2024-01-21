use std::error::Error;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust cat")]
pub struct  Config {
    /// input file
    files: Vec<String>,

    /// Number lines
    #[arg(short='n', long="number")]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short='b', long="number-nonblank")]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    Ok(Config::parse())
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        println!("{}", filename);
    }
    Ok(())
}
