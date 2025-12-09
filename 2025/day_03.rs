
fn find_biggest_combination(n_digits: usize, mut slice: &[u8]) -> usize {
    let mut sol = 0;
    for i in (0..n_digits).rev() {
        let (best_index, n) = slice
            .iter()
            .enumerate()
            .rev()
            .skip(i)
            .max_by_key(|a| a.1)
            .unwrap();

        slice = &slice[best_index+1..];
        sol = sol * 10 + (n - b'0') as usize;
    }
    sol
}

pub fn solve<I, S>(input: I) -> (usize, usize)
where
    I: Iterator<Item = S>,
    S: AsRef<str>
{
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input {
        let line = line.as_ref();
        part1 += find_biggest_combination(2, line.as_bytes());
        part2 += find_biggest_combination(12, line.as_bytes());
    }

    (part1, part2)
}

fn main() {
    let input = aoc::get_input_lines();
    aoc::run(|| solve(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
    let (part1, part2) = super::solve("987654321111111
811111111111119
234234234234278
818181911112111".lines());
        assert_eq!(part1, 357);
        assert_eq!(part2, 3121910778619);
    }
}
