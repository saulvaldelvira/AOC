use core::str;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

#[macro_export]
macro_rules! solution {
    () => {
        pub fn main() {
            let arg = std::env::args().skip(1).next();
            let fname = arg.as_deref().unwrap_or("input.txt");
            let src = std::fs::read_to_string(fname)
                              .unwrap_or_else(|_| {
                                  eprintln!("File \"{fname}\" not found");
                                  std::process::exit(1);
                              });

            let start = std::time::Instant::now();
            let (part1,part2) = solve(&src);
            let end = start.elapsed();
            println!("Part 1: {part1}");
            println!("Part 2: {part2}\n");
            let us = (end.as_nanos() as f64 / 1000.0).round();
            let ms = end.as_millis();
            if us > 1000.0 {
                println!("Time: {ms} ms ({us} \u{00B5}s)");
            } else {
                println!("Time: {us} \u{00B5}s");
            }
        }
    };
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

pub fn input_to_char_array(src: &str) -> Box<[Box<[char]>]> {
    src.lines().filter(|l| !l.is_empty()).map(|l| {
        l.chars().collect::<Vec<_>>().into_boxed_slice()
    }).collect::<Vec<_>>().into_boxed_slice()
}

pub fn input_to_byte_array(src: &str) -> Box<[Box<[u8]>]> {
    src.lines().filter(|l| !l.is_empty()).map(|l| {
        l.bytes().collect::<Vec<_>>().into_boxed_slice()
    }).collect::<Vec<_>>().into_boxed_slice()
}

#[allow(private_bounds)]
pub mod point {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    trait Numeric:
        Sized + Clone + Copy + Add<Output = Self> + Sub<Output = Self> + Mul + Div + Ord +
        AddAssign + MulAssign + SubAssign + DivAssign {}

    impl<T> Numeric for T
    where T : Sized + Clone + Copy + Add<Output = Self> + Sub<Output = Self> + Mul + Div + Ord +
              AddAssign + MulAssign + SubAssign + DivAssign {}

    #[derive(Debug,Hash,Eq,PartialEq,Clone,Copy,PartialOrd,Ord)]
    pub struct Point<T: Numeric>{
        pub x: T,
        pub y: T,
    }

    impl<T: Numeric> Point<T> {
        pub fn get_earring(&self, o: &Self) -> Self {
            let (a,b) = if *self > *o { (o,self) } else { (self, o) };
            Point {
                x: b.x - a.x,
                y: b.y - a.y,
            }
        }
        pub fn is_withing(&self, top_left: &Self, bottom_right: &Self) -> bool {
            self.x >= top_left.x
            && self.y >= top_left.y
            && self.x <= bottom_right.x
            && self.y <= bottom_right.y
        }
    }

    impl<T: Numeric> Sub for Point<T> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }

    impl<T: Numeric> Add for Point<T> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl<T: Numeric> Add<(T,T)> for Point<T> {
        type Output = Self;

        fn add(self, rhs: (T,T)) -> Self::Output {
            Point {
                x: self.x + rhs.0,
                y: self.y + rhs.1,
            }
        }
    }

    impl<T: Numeric> AddAssign for Point<T> {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x;
            self.y += rhs.y;
        }
    }

    impl<T: Numeric> SubAssign for Point<T> {
        fn sub_assign(&mut self, rhs: Self) {
            self.x -= rhs.x;
            self.y -= rhs.y;
        }
    }
}
