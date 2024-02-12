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
    println!("{:?}", config);
    Ok(())
}
