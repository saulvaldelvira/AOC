use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn solve(input: &[String]) -> (usize, usize) {
    let mut changes = HashMap::new();
    let mut names = HashSet::new();

    for line in input {
        let (name, rest) = line.split_once(" ").unwrap();

        let mut sign = 1;
        let rest = if let Some(rest) = rest.strip_prefix("would gain ") {
            rest
        } else if let Some(rest) = rest.strip_prefix("would lose ") {
            sign = -1;
            rest
        } else {
            unreachable!()
        };

        let (amount, target) = rest.split_once(" happiness units by sitting next to ").unwrap();
        let amount = amount.parse::<isize>().unwrap() * sign;
        let target = target.strip_suffix('.').unwrap();

        if !names.contains(name) {
            names.insert(name);
        }
        if !names.contains(target) {
            names.insert(target);
        }

        changes.insert((name, target), amount);
    }

    fn happines_for(arrangement: &[&str], values: &HashMap<(&str, &str), isize>) -> isize {
        let mut acc = 0;
        for (a, b) in arrangement.iter().zip(arrangement.iter().skip(1)) {
            acc += values.get(&(a, b)).unwrap_or(&0);
            acc += values.get(&(b, a)).unwrap_or(&0);
        }
        let a = arrangement.first().unwrap();
        let b = arrangement.last().unwrap();
        acc += values.get(&(a, b)).unwrap_or(&0);
        acc += values.get(&(b, a)).unwrap_or(&0);
        acc
    }

    fn max_happiness(
        slice: &HashSet<&str>,
        changes: &HashMap<(&str, &str), isize>
    ) -> usize
    {
        slice.iter().copied()
            .permutations(slice.len())
            .map(|p| {
                happines_for(&p, changes)
            })
        .max()
        .unwrap() as usize
    }

    let part1 = max_happiness(&names, &changes);
    names.insert("me");
    let part2 = max_happiness(&names, &changes);

    (part1, part2)
}

fn main() {
    let input: Vec<String> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let (part1, _) = super::solve("\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
.lines()
.map(str::to_string)
.collect::<Vec<_>>()
.as_slice());

        assert_eq!(part1, 330);
    }
}
