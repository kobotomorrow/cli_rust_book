use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

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
    let files = if config.files.is_empty() { vec![String::from("-")] } else { config.files };
    for filename in files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                if let Err(err) = read(&mut file, config.number_lines, config.number_nonblank_lines) {
                    eprintln!("Failed to read {}: {}", filename, err);
                }
            }
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

fn read(reader: &mut Box<dyn BufRead>, n: bool, b: bool) -> MyResult<()> {
    let mut empty_line_count = 0;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if b && line.is_empty() {
            empty_line_count += 1;
            println!();
            continue;
        }

        let line_num = i + 1 - empty_line_count;
        if n || b {
            println!("{:>6}\t{}", line_num, line);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}
