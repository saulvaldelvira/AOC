use core::ops::{AddAssign, SubAssign};

use aoc::get_input_lines;

fn solve<S: AsRef<str>>(it: impl Iterator<Item = S>) -> (usize, usize) {
    let mut dial = 50_i16;
    let mut part1 = 0;
    let mut part2 = 0;

    for line in it {
        let (direction, amount) = line.as_ref().split_at(1);
        let amount = amount.parse::<i16>().expect("Expected a valid number");

        let func: fn (&mut i16, i16) =
        match direction.as_bytes()[0] {
            b'R' => i16::add_assign,
            b'L' => i16::sub_assign,
            c => unreachable!("The only valid values are 'R' and 'L': {}", char::from(c))
        };

        for _ in 0..amount {
            func(&mut dial, 1);
            if dial >= 100 {
                dial = 0;
            } else if dial < 0 {
                dial = 99
            }
            if dial == 0 {
                part2 += 1;
            }
        }

        if dial == 0 {
            part1 += 1;
        }
    }

    (part1, part2)
}

fn main() {
    let input = get_input_lines();
    aoc::run(|| solve(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
    let (part1, part2) = super::solve("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82".lines());
        assert_eq!(part1, 3);
        assert_eq!(part2, 6);
    }
}
