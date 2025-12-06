use std::fmt::Write;

fn look_and_say(num: &str) -> String {
    let mut result = String::new();

    let mut digits = num.chars().map(|c| c as u8 - b'0');
    let mut curr_digit = digits.next().unwrap();
    let mut count = 1;
    for digit in digits {
        if digit != curr_digit {
            write!(result, "{count}{curr_digit}").unwrap();
            curr_digit = digit;
            count = 1;

        } else {
            count += 1;
            assert!(count <= 9);
        }
    }
    write!(result, "{count}{curr_digit}").unwrap();

    result
}

fn solve(input: &str) -> (usize, usize) {
    let mut num = input.to_string();

    let mut part1 = 0;
    for i in 1..=50 {
        num = look_and_say(&num);
        if i == 40 {
            part1 = num.len();
        }
    }

    let part2 = num.len();
    (part1, part2)
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use crate::look_and_say;

    #[test]
    fn example() {
        let mut n = "1".to_string();
        const EXPECTED: &[&str] = &[ "11", "21", "1211", "111221" ];
        for exp in EXPECTED {
            n = look_and_say(&n);
            assert_eq!(n.as_str(), *exp);
        }
    }
}
