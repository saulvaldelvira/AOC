use itertools::Itertools;

fn solve(weights: &[u64], n_packages: u64) -> u64 {
    for i in 1..weights.len() {
        let expected_sum = weights.iter().sum::<u64>() / n_packages;
        let min = weights.iter().combinations(i)
            .filter_map(|comb| {
                let sum = comb.iter().copied().sum::<u64>();
                (sum == expected_sum).then(|| comb.iter().copied().product())
            })
            .min();
        if let Some(min) = min {
            return min
        }
    }
    unreachable!("The input should guarantee that a valid combination exists");
}


fn main() {
    let input: Vec<u64> = aoc::get_input_file_lines()
        .map(|l| l.parse().unwrap()).collect();
    aoc::run(|| (solve(&input, 3), solve(&input, 4)));
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        const EXAMPLE_INPUT: &[u64] = &[1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
        let pt1 = solve(EXAMPLE_INPUT, 3);
        assert_eq!(pt1, 99);
        let pt2 = solve(EXAMPLE_INPUT, 4);
        assert_eq!(pt2, 44);
    }
}
