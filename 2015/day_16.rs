use std::collections::HashMap;

fn solve(input: &[String], expected: &HashMap<&str, u16>) -> (usize, usize) {
    let aunts: Vec<_> =
        input.iter()
        .enumerate()
        .map(|(i, line)| {
            let (_, rest) = line.split_once(": ").unwrap();

            let mut values = rest.trim().split(", ");
            let mut parse_next = || -> Option<(&str, u16)> {
                let (t, am) = values.next()?
                    .split_once(": ").unwrap();
                Some((t, am.parse::<u16>().unwrap()))
            };

           let mut aunt = HashMap::new();
           while let Some((ty, am)) = parse_next() {
               aunt.insert(ty, am);
           }
           aunt.insert("id", i as u16 + 1);

           aunt
        })
        .collect();

    let part1 = *aunts.iter().find(|a| {
        for (key, val) in *a {
            if *key == "id" { continue }
            let expected = expected.get(key).unwrap();
            if val != expected { return false }
        }
        true
    }).unwrap().get("id").unwrap();

    let part2 = *aunts.iter().find(|a| {
        a.iter().all(|(key, val)| {
            if *key == "id" { return true }
            let expected = expected.get(key).unwrap();
            match *key {
                "cats" | "trees" => val > expected,
                "pomeranians" | "goldfish" => val < expected,
                _ => val == expected
            }
        })
    }).unwrap().get("id").unwrap();

    (part1 as _, part2 as _)
}

fn main() {
    let input: Vec<String> = aoc::get_input_file_lines().collect();
    let mut expected = HashMap::new();
    for (k, v) in [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ] {
        expected.insert(k, v);
    }
    aoc::run(|| solve(&input, &expected));
}
