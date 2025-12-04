use std::thread;

fn find_permutations(mut max: usize, acc: usize, n_digits: u32, slice: &[u8]) -> usize {
    if slice.len() < n_digits as usize { return max }
    let mut max_possible = acc * 10_usize.pow(n_digits);
    for i in 0..n_digits {
        max_possible += 9 * 10_usize.pow(i);
    }
    if max_possible < max { return max }
    for i in 0..slice.len() {
        let m = acc * 10 + (slice[i] - b'0') as usize;
        if n_digits == 1 {
            max = usize::max(max, m);
        } else {
            max = find_permutations(max, m, n_digits - 1, slice.get(1 + i..).unwrap_or(&[]));
        }
    }
    max
}

fn get_max_combination(line: &str) -> (usize, usize) {
    let part1 = find_permutations(0, 0, 2, line.as_bytes());
    let part2 = find_permutations(0, 0, 12, line.as_bytes());
    (part1, part2)
}

pub fn solve<I, S>(input: I) -> (usize, usize)
where
    I: Iterator<Item = S>,
    S: AsRef<str>
{
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input {
        let (p1, p2) = get_max_combination(line.as_ref());
        part1 += p1;
        part2 += p2;
    }

    (part1, part2)
}

pub fn solve_pararell(input: &[String]) -> (usize, usize) {
    thread::scope(|scope| {
        let threads: Vec<_> = input.iter().map(|line| {
            scope.spawn(|| get_max_combination(line))
        }).collect();

        let mut part1 = 0;
        let mut part2 = 0;

        for t in threads {
            let (p1, p2) = t.join().unwrap();
            part1 += p1;
            part2 += p2;
        }

        (part1, part2)
    })

}

fn main() {
    let input: Vec<String> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve_pararell(&input));
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
