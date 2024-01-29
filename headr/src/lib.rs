use std::error::Error;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust head")]
pub struct Config {
    /// input file
    files: Vec<String>,
    
    /// Number of lines
    #[arg(short='n', long="lines" , default_value_t = 10, value_parser = clap::value_parser!(u32).range(1..), group = "input")]
    lines: u32,

    /// Number of bytes
    #[arg(short='c', long="bytes", value_parser = clap::value_parser!(u32).range(1..), group = "input")]
    bytes: Option<u32>,
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
    println!("{:?}", config);
    Ok(())
}
