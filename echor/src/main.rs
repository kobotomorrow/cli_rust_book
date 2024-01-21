use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust echo")]
struct Args {
    /// Input text
    text: String,
    
    /// Optional texts
    optional_texts: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}{}{}",
        args.text,
        if args.optional_texts.is_empty() { "" } else { " "},
        args.optional_texts.join(" "),
        if args.omit_newline { "" } else { "\n" });
}
