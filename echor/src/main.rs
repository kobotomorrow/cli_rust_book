use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust echo")]
struct Args {
    /// Input text
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!("{}{}", args.text.join(" "), if args.omit_newline { "" } else { "\n" });
}
