use std::{fs::File, io::{BufReader, Read}};

fn main() {
    let file = File::open("src/bin/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut output = String::new();
    _ = reader.read_to_string(&mut output);

    println!("{:?}", output);
}