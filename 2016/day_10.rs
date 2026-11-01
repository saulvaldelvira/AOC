use std::collections::HashMap;

enum Dest {
    Bot(usize),
    Output(usize),
}

struct BotRule {
    low_to: Dest,
    high_to: Dest,
}

fn give_to(val: i32, vals: &mut (i32, i32)) {
    if vals.0 == -1 {
        vals.0 = val;
    } else {
        vals.1 = val;
    }
}


fn parse(lines: &[impl AsRef<str>]) -> (HashMap<usize, BotRule>, HashMap<usize, (i32, i32)>) {
    let mut rules = HashMap::new();
    let mut values = HashMap::new();

    for line in lines.iter().map(AsRef::as_ref) {
        if let Some(line) = line.strip_prefix("value ") {
            let (val, botid) = line.split_once(" goes to bot ").unwrap();
            let val = val.parse::<i32>().unwrap();
            let botid = botid.parse::<usize>().unwrap();
            let vals = values.entry(botid).or_insert((-1, -1));
            give_to(val, vals);
        } else {
            let line = line.strip_prefix("bot ").unwrap();
            let (srcid, rest) = line.split_once(" gives low to ").unwrap();
            let srcid = srcid.parse::<usize>().unwrap();
            let (lowdst, highdst) = rest.split_once(" and high to ").unwrap();
            fn parse(s: &str) -> Dest {
                let (kind, val) = s.split_once(" ").unwrap();
                let val = val.parse().unwrap();
                match kind {
                    "bot" => Dest::Bot(val),
                    "output" => Dest::Output(val),
                    _ => unreachable!()
                }
            }
            let rule = BotRule {
                low_to: parse(lowdst),
                high_to: parse(highdst),
            };
            rules.insert(srcid, rule);
        }
    }

    (rules, values)
}

fn solve(rules: &HashMap<usize, BotRule>, vals: &mut HashMap<usize, (i32, i32)>, tmin: i32, tmax: i32) -> (usize, i32) {
    let mut outputs = HashMap::new();
    let mut pt1 = 0;
    while let Some(id) = vals.iter().find_map(|(&b, &vals)| (vals.0 != -1 && vals.1 != -1).then_some(b)) {
        let (a, b) = vals.get_mut(&id).unwrap();
        let min = *a.min(b);
        let max = *a.max(b);
        if min == tmin && max == tmax {
            pt1 = id;
        }
        *a = -1;
        *b = -1;

        let BotRule { low_to, high_to } = rules.get(&id).unwrap();

        for (val, to) in [(min, low_to), (max, high_to)] {
            match to {
                Dest::Bot(id) => {
                    let e = vals.entry(*id).or_insert((-1, -1));
                    give_to(val, e);
                }
                Dest::Output(out) => {
                    outputs.insert(out, val);
                }
            }

        }
    }

    let pt2 = outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap();
    (pt1, pt2)
}

pub fn main() {
    let input: Vec<_> = aoc::get_input_file_lines().collect();
    aoc::run(|| {
        let (rules, mut vals) = parse(&input);
        solve(&rules, &mut vals, 17, 61)
    });

}

#[cfg(test)]
mod test {
    use crate::{parse, solve};

    #[test]
    fn test() {
        let (rules, mut vals) = parse("\
value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2
".lines().collect::<Vec<_>>().as_slice());
        let (pt1, _) = solve(&rules, &mut vals, 2, 5);
        assert_eq!(pt1, 2);
    }
}
