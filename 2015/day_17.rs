use std::collections::HashMap;

use itertools::Itertools;

fn solve(input: &[String], amount: u16) -> (usize, usize) {
    let containers: Vec<u16> = input.iter()
        .map(|n| n.parse::<u16>().unwrap())
        .collect();

    let combinations = containers.iter().powerset()
            .filter(|set| {
                set.iter().copied().sum::<u16>() == amount
            });

    let mut part1 = 0;
    let mut min_len = usize::MAX;
    let mut count_by_len = HashMap::<usize, usize>::new();
    for comb in combinations {
        part1 += 1;

        min_len = min_len.min(comb.len());
        *count_by_len.entry(comb.len()).or_default() += 1;
    }
    let part2 = count_by_len.get(&min_len).unwrap();

    (part1, *part2)
}

fn main() {
    let input: Vec<String> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve(&input, 150));
}

#[cfg(test)]
mod tests {

    #[test]
    fn example() {
        let input = "\
20
15
10
5
5".lines().map(str::to_string).collect::<Vec<_>>();
        let (part1, part2) = super::solve(&input, 25);
        assert_eq!(part1, 4);
        assert_eq!(part2, 3);
    }
}
