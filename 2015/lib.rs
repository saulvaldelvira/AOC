use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

pub fn get_input_file() -> BufReader<File> {
    let f = File::open("input.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });
    BufReader::new(f)
}

pub fn get_input_file_lines() -> impl Iterator<Item = String> {
    get_input_file()
    .lines()
    .map_while(Result::ok)
    .filter(|l| !l.is_empty())
}

