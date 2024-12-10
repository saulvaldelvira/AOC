
#[inline(always)]
fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn can_be_solved(is_part2: bool, expected: u64, carry: u64, vals: &[u64]) -> bool {
    if carry > expected { return false }
    if vals.is_empty() { return carry == expected  }

    let mul = can_be_solved(is_part2, expected, carry * vals[0], &vals[1..]);
    let sum = can_be_solved(is_part2, expected, carry + vals[0], &vals[1..]);
    let concat = is_part2 && can_be_solved(is_part2, expected, concat(carry, vals[0]), &vals[1..]);
    mul || sum || concat
}

fn solve(src: &str) -> (u64,u64) {
    src.lines()
   .filter(|l| !l.is_empty())
   .filter_map(|line| {
        let mut line = line.split(':');
        let expected: u64 = line.next().unwrap().parse().unwrap();
        let values = line.next().unwrap();
        let values: Vec<u64> =
            values
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let part_1 = can_be_solved(false, expected, values[0], &values[1..]);
        let part_2 = can_be_solved(true, expected, values[0], &values[1..]);

        if part_1 || part_2 {
            Some((
                    if part_1 { expected } else { 0 },
                    if part_2 { expected } else { 0 },
            ))
        } else {
            None
        }
    }).fold((0,0), |mut acc, val| {
        acc.0 += val.0;
        acc.1 += val.1;
        acc
    })
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn example() {
        const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let (part1,part2) = solve(INPUT);
        assert_eq!(part1, 3749);
        assert_eq!(part2, 11387);
    }
}
