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
