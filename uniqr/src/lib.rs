use std::{fs::File, io::{self, BufRead, BufReader, Write}, ops::Add};

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
    let out_file = if let Some(filename) = config.out_file {
        Some(File::create(filename)?)
    } else {
        None
    };

    let mut cur_line = String::new();
    let mut next_line = String::new();
    let mut count = 1;
    let mut out_text = String::new();

    let bytes = file.read_line(&mut cur_line)?;
    if bytes == 0 {
        return Ok(())
    }
    loop {
        let bytes = file.read_line(&mut next_line)?;
        if bytes == 0 {
            let text = if config.count {
                format!("{:>4} {}", count, cur_line)
            } else {
                format!("{}", cur_line)
            };
            out_text = out_text.add(&text);
            break;
        }
        let cur = cur_line.lines().collect::<String>();
        let next = next_line.lines().collect::<String>();
        if cur != next {
            let text = if config.count {
                format!("{:>4} {}", count, cur_line)
            } else {
                format!("{}", cur_line)
            };
            out_text = out_text.add(&text);
            cur_line = next_line.clone();
            count = 1;
        } else {
            count += 1;
        }
        next_line.clear();
    }
    if let Some(mut file) = out_file {
        let _ = file.write_all(out_text.as_bytes());
    } else {
        print!("{}", out_text);
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
