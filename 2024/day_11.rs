use std::collections::HashMap;

#[inline]
fn split_int(n: i64) -> (i64,i64) {
    let n_str = n.to_string();
    let uphalf = n_str[0..n_str.len() / 2].parse::<i64>().unwrap();
    let bottomhalf = n_str[n_str.len() / 2..].parse::<i64>().unwrap();
    (uphalf,bottomhalf)
}

fn solve(src: &str) -> (usize,usize) {
    let stones: Vec<_> =
        src.split_whitespace()
           .map(|s| s.parse::<i64>().unwrap())
           .collect();

    let mut counter = HashMap::new();
    for stone in &stones {
        counter.insert(*stone, 1);
    }

    fn blink(counter: &mut HashMap<i64, usize>) {
        let mut new_counter = HashMap::with_capacity(counter.capacity());

        for (&stone, count) in counter.iter() {
                let ndigits = stone.checked_ilog10().unwrap_or(0) + 1;
                if stone == 0 {
                    *new_counter.entry(1).or_insert(0) += count;
                } else if ndigits % 2 == 0 {
                    let (uphalf,bottomhalf) = split_int(stone);
                    *new_counter.entry(uphalf).or_insert(0) += count;
                    *new_counter.entry(bottomhalf).or_insert(0) += count;
                } else if ndigits % 2 != 0 {
                    let n = stone * 2024;
                    *new_counter.entry(n).or_insert(0) += count;
                }

        }

        *counter = new_counter;
    }

    let mut part1 = 0;
    for i in 1..=75 {
        blink(&mut counter);
        if i == 25 {
            part1 = counter.values().sum();
        }
    }
    let part2 = counter.values().sum();

    (part1,part2)
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let (part1,part2) = solve("125 17");
        assert_eq!(part1, 55312);
        assert_eq!(part2, 65601038650482);
    }
}
