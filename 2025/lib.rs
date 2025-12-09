use core::time::Duration;
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

#[macro_export]
macro_rules! dprintln {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($t)*)
    };
}

pub fn measure<T>(func: impl FnOnce() -> T) -> (T, Duration) {
    let start = std::time::Instant::now();
    let ret = func();
    let end = start.elapsed();
    (ret, end)
}

pub fn run(solve: impl FnOnce() -> (usize, usize)) {
    let start = std::time::Instant::now();
    let (part1,part2) = solve();
    let end = start.elapsed();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
    print_time(end);
}

pub fn print_time(t: Duration) {
    let ms = t.as_millis();
    if ms > 0 {
        println!("{ms} ms");
    } else {
        println!("{} \u{00B5}s", t.as_micros());
    }
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

pub fn get_input_lines() -> impl Iterator<Item = String> {
    get_input_file()
    .lines()
    .map_while(Result::ok)
    .filter(|l| !l.is_empty())
}

pub fn get_input_matrix() -> Vec<Vec<u8>> {
    get_input_lines()
    .map(|line| line.into_bytes())
    .collect()
}

/// Iterates over all possible pairs of elements of a slice
///
/// # Example
/// ```
/// use aoc::permutations;
///
/// let arr = [1, 2, 3, 4];
/// let mix: Vec<_> = permutations(&arr)
///     .map(|(&a, &b)| (a, b))
///     .collect();
///
/// assert_eq!(&mix, &[
///     (1, 2), (1, 3), (1, 4),
///     (2, 3), (2, 4), (3, 4)
/// ])
/// ```
pub fn permutations<T>(arr: &[T]) -> impl Iterator<Item = (&T, &T)> {
    (0..arr.len())
    .flat_map(move |i| {
        (i+1..arr.len())
        .map(move |j| {
            (&arr[i], &arr[j])
        })
    })
}

/// Iterates the slice in chunks of 2, wrapping the last element with the first one
///
/// # Example
/// ```
/// use aoc::pairs_wrap;
///
/// let arr = [1, 2, 3, 4, 5];
/// let mix: Vec<_> = pairs_wrap(&arr)
///     .map(|(&a, &b)| (a, b))
///     .collect();
///
/// assert_eq!(&mix, &[
///     (1, 2), (2, 3), (3, 4),
///     (4, 5), (5, 1)
/// ])
/// ```
pub fn pairs_wrap<T>(arr: &[T]) -> impl Iterator<Item = (&T, &T)> {
    let second = arr.iter().skip(1).chain([&arr[0]]);
    arr.iter().zip(second)
}
