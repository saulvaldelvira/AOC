use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process;

#[macro_export]
macro_rules! num_digits {
    ($n:expr) => {
        match $n.checked_ilog10() {
            Some(n) => n + 1,
            None => 1,
        }
    };
}

pub fn run(solve: impl FnOnce() -> (usize, usize)) {
    let start = std::time::Instant::now();
    let (part1,part2) = solve();
    let end = start.elapsed();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    let ms = end.as_millis();
    if ms > 0 {
        println!("Time: {ms} ms");
    } else {
        println!("Time: {} \u{00B5}s", end.as_micros());
    }
}

pub fn get_input_file() -> BufReader<File> {
    let f = File::open("input.txt").unwrap_or_else(|err| {
        eprintln!("Error opening file: {err}");
        process::exit(1);
    });
    BufReader::new(f)
}

pub fn get_input_string() -> String {
    let mut s = String::new();
    get_input_file().read_to_string(&mut s).unwrap();
    s
}

pub fn get_input_file_bytes() -> impl Iterator<Item = u8> {
    get_input_file()
    .bytes()
    .map_while(Result::ok)
}

pub fn get_input_file_lines() -> impl Iterator<Item = String> {
    get_input_file()
    .lines()
    .map_while(Result::ok)
    .filter(|l| !l.is_empty())
}

