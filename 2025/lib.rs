use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process;

pub fn run(solve: impl FnOnce() -> (usize, usize)) {
    let start = std::time::Instant::now();
    let (part1,part2) = solve();
    let end = start.elapsed();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}\n");
    let ms = end.as_millis();
    println!("Time: {ms} ms");
}

pub fn get_input_string() -> String {
    let mut s = String::new();
    get_input_file().read_to_string(&mut s).unwrap();
    s
}

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
