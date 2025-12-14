
fn solve(num: usize) -> (usize, usize) {

    let mut houses = vec![0; num / 10];

    for elve in 1..num/10 {
        for h in (elve..num/10).step_by(elve) {
            houses[h - 1] += elve * 10;
        }
    }

    let part1 = houses.iter().enumerate().find(|(_, n)| **n >= num).unwrap().0 + 1;
    houses.iter_mut().for_each(|n| *n = 0 );

    for elve in 1..num/10 {
        for h in (elve..num/10).step_by(elve).take(50) {
            houses[h - 1] += elve * 11;
        }
    }

    let part2 = houses.iter().enumerate().find(|(_, n)| **n >= num).unwrap().0 + 1;

    (part1, part2)
}

fn main() {
    let input = std::env::args()
        .nth(1).unwrap()
        .parse::<usize>().unwrap();
    aoc::run(|| solve(input));
}
