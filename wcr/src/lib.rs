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

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
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
    let mut line_count = 0;
    let mut word_count = 0;
    let mut byte_count = 0;
    let mut char_count = 0;
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let file_info = count(file)?;
                if config.lines {
                    print!("{:>8}", file_info.num_lines);
                }
                if config.words {
                    print!("{:>8}", file_info.num_words);
                }
                if config.bytes {
                    print!("{:>8}", file_info.num_bytes);
                }
                if config.chars {
                    print!("{:>8}", file_info.num_chars);
                }
                if filename != "-" {
                    println!(" {}", filename);
                } else {
                    println!();
                }

                line_count += file_info.num_lines;
                word_count += file_info.num_words;
                byte_count += file_info.num_bytes;
                char_count += file_info.num_chars;
            }
        }
    }
    if config.files.len() > 1 {
        if config.lines {
            print!("{:>8}", line_count);
        }
        if config.words {
            print!("{:>8}", word_count);
        }
        if config.bytes {
            print!("{:>8}", byte_count);
        }
        if config.chars {
            print!("{:>8}", char_count);
        }
        println!(" total");
    };
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    loop {
        let mut line = String::new();
        let len = file.read_line(&mut line)?;
        if len == 0 {
            break;
        }

        num_lines += 1;
        num_bytes += len;
        num_chars = line.chars().count();
        num_words += line.split_whitespace().count();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
