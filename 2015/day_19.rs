use core::usize;
use std::collections::{HashMap, HashSet, VecDeque};

fn invert_grammar<'a>(rules: HashMap<&'a str, Vec<&'a str>>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut new = HashMap::<&str, Vec<&str>>::new();
    for (k, v) in rules {
        for v in v {
            new.entry(v).or_default()
                .push(k);
            }
    }
    new
}

fn solve(lines: &[String]) -> (usize, usize) {

    let mut rules = HashMap::<&str, Vec<&str>>::new();
    let mut longest = 0;
    for rule in &lines[0..lines.len() - 1] {
        if rule.is_empty() { continue }

        let (rule, output) = rule.split_once(" => ").unwrap();
        rules.entry(rule).or_default()
            .push(output);
        longest = longest.max(rule.len());
    }

    let mut unique = HashSet::new();
    let input = lines.last().unwrap();

    for n in 1..=longest {
        let windows = input.as_bytes().windows(n).enumerate().map(|(i, w)| {
            (i, str::from_utf8(w).unwrap())
        });
        for (i, w) in windows {
            if let Some(rules) = rules.get(w) {
                for out in rules {
                    let mut new_input = input.clone();
                    new_input.drain(i..i + n).for_each(|_| {});
                    new_input.insert_str(i, out);
                    unique.insert(new_input);
                }
            }
        }
    }

    let part1 = unique.len();

    let rules = invert_grammar(rules);
    let longest = rules.keys().map(|k| k.len()).max().unwrap();

    let mut to_try = VecDeque::new();
    to_try.push_back((input.clone(), 0));
    let mut cache = HashMap::<String, u16>::new();
    cache.insert(input.clone(), 0);

    let part2 = loop {
        let Some((input, steps)) = to_try.pop_back() else { panic!("No solution") };
        if input == "e" {
            break steps as usize;
        }
        for n in 1..=longest {
            let windows = input.as_bytes().windows(n).enumerate().map(|(i, w)| {
                (i, str::from_utf8(w).unwrap())
            });
            for (i, w) in windows {
                if let Some(rules) = rules.get(w) {
                    for out in rules {
                        let mut new_input = input.clone();
                        new_input.drain(i..i + n).for_each(|_| {});
                        new_input.insert_str(i, out);
                        match cache.get_mut(&new_input) {
                            Some(n) => {
                                if *n > steps + 1 {
                                    *n = steps + 1;
                                }
                            },
                            None => {
                                cache.insert(new_input.clone(), steps + 1);
                            }
                        }

                        to_try.push_back((new_input, steps + 1));
                    }
                }
            }
        }
    };

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
        let input: Vec<_> = "\
e => H
e => O
H => HO
H => OH
O => HH

HOH
".lines().map(str::to_string).collect();

        let (part1, part2) = super::solve(&input);

        assert_eq!(part1, 4);
        assert_eq!(part2, 3);

    }

    #[test]
    fn example2() {
        let input: Vec<_> = "\
e => H
e => O
H => HO
H => OH
O => HH

HOHOHO
".lines().map(str::to_string).collect();

        let (part1, part2) = super::solve(&input);

        assert_eq!(part1, 7);
        assert_eq!(part2, 6);

    }
}
