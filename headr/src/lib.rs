use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust head")]
pub struct Config {
    /// input file
    files: Vec<String>,

    /// Number of lines
    #[arg(short='n', long="lines" , default_value_t = 10, value_parser = clap::value_parser!(u64).range(1..), group = "input")]
    lines: u64,

    /// Number of bytes
    #[arg(short='c', long="bytes", value_parser = clap::value_parser!(u64).range(1..), group = "input")]
    bytes: Option<u64>,
}

// fn parse_positive_int(val: usize) -> MyResult<usize> {
//     if val == 0 {
//         Err(From::from("0"))
//     } else {
//         Ok(val)
//     }
// }

// #[test]
// fn test_parse_positive_int() {
//     let res = parse_positive_int(3);
//     assert!(res.is_ok());

//     let res = parse_positive_int(0);
//     assert!(res.is_err());
//     assert_eq!(res.unwrap_err().to_string(), "0".to_string());
// }

pub fn get_args() -> MyResult<Config> {
    let mut config = Config::parse();
    if config.files.is_empty() {
        config.files = vec![String::from("-")]
    }
    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    let filenames = config.files;
    let num_files = filenames.len();

    for (i, filename) in filenames.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {} <==", if i > 0 { "\n" } else { "" }, filename);
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes);
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
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
