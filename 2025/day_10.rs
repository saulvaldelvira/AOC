use std::collections::{HashSet, VecDeque};
use aoc::get_input_lines;

struct Button {
    effect: u64,
}

impl Button {
    const fn affects(&self, jold_id: usize) -> bool {
        (self.effect & (1 << jold_id)) != 0
    }
}

struct Machine {
    lights_target: u64,
    joltages_target: Vec<u16>,
    buttons: Vec<Button>
}

impl Machine {
    fn solve(&self) -> usize {
        let mut cache = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((0, 0));

        loop {
            let (state, count) = queue.pop_front().unwrap();

            if state == self.lights_target {
                return count
            }

            for b in &self.buttons {
                let value = state ^ b.effect;
                if cache.insert(value) {
                    queue.push_back((value, count + 1));
                }
            }
        }
    }

    fn solve_pt2(&self) -> usize {
        /* Taken from: https://nickymeuleman.netlify.app/blog/aoc2025-day10/ */
        use good_lp::*;

        let mut vars = variables!();
        let presses: Vec<Variable> = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        let total_presses: Expression = presses.iter().sum();
        let mut problem = vars.minimise(total_presses).using(default_solver);

        for (jolt, &target) in self.joltages_target.iter().enumerate() {
            let mut expr = Expression::from(0);

            for (i, but) in self.buttons.iter().enumerate() {
                if but.affects(jolt) {
                    expr += presses[i];
                }
            }

            problem.add_constraint(expr.eq(target));
        }

        let solution = problem.solve().unwrap();

        presses
            .iter()
            .map(|v| solution.value(*v).round() as usize)
            .sum()
    }
}

fn solve(lines: &[impl AsRef<str>]) -> (usize, usize) {
    let machines: Vec<Machine> = lines.iter()
        .map(|line| {
            let mut line = line.as_ref();
            line = line.strip_prefix("[").unwrap();
            let (lights, rest) = line.split_once("] ").unwrap();
            let (buttons, joltages) = rest.split_once(" {").unwrap();

            let joltages_target = joltages.strip_suffix('}').unwrap().split(",").map(|n| {
                n.parse::<u16>().unwrap()
            })
            .collect();

            let lights_target = lights.chars().rev().map(|l| u8::from(l == '#'))
                .fold(0_u64, |acc, val| {
                    (acc << 1) | val as u64
                });

            let buttons = buttons.split_whitespace().map(|b| {
                let b = b.strip_prefix("(").unwrap().strip_suffix(")").unwrap();
                let effect = b.split(",").map(|n| n.parse::<usize>().unwrap())
                    .fold(0_u64, |acc, n| {
                        acc | (1 << n)
                    });
                Button { effect }
            }).collect();

            Machine {
                lights_target,
                joltages_target,
                buttons
            }
        })
        .collect();

    let part1 = machines.iter().map(|m| m.solve()).sum();

    let part2 = machines.iter().map(|m| m.solve_pt2()).sum::<usize>();
    (part1, part2)
}

fn main() {
    let input: Vec<_> = get_input_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let (part1, part2) = solve(&"\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
.lines()
.collect::<Vec<_>>());
        assert_eq!(part1, 7);
        assert_eq!(part2, 33);
    }
}

/* Initial brute-force implementation
#[allow(
    clippy::items_after_test_module,
    unused
)]
impl Button {
    fn apply_joltage(&self, jolts: &mut [u16]) {
        for (i, jolt) in jolts.iter_mut().enumerate() {
            if (self.effect & (1 << i)) != 0 {
                *jolt += 1;
            }
        }
    }

    fn undo_joltage(&self, jolts: &mut [u16]) {
        for (i, jolt) in jolts.iter_mut().enumerate() {
            if (self.effect & (1 << i)) != 0 {
                *jolt -= 1;
            }
        }
    }
}

#[allow(unused)]
impl Machine {
    fn solve_2_brute_force(&self) -> usize {
        let mut cache: HashMap<u64, usize> = HashMap::new();
        let mut queue = BinaryHeap::new();

        #[derive(PartialEq, Eq)]
        struct State {
            joltages: Vec<u16>,
            n_push: usize,
            diff_target: u16,
        }

        impl State {
            fn new(joltages: Vec<u16>, n_push: usize, target: &[u16]) -> Self {
                let mut diff_target = 0;
                for (sj, oj) in joltages.iter().zip(target.iter()) {
                    debug_assert!(oj >= sj);
                    diff_target += oj - sj;
                }
                Self { joltages, n_push, diff_target }

            }

            fn cost(&self) -> u16 {
                self.n_push as u16 + self.diff_target
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.cost().cmp(&self.cost())
            }
        }

        queue.push(State::new(vec![0; self.joltages_target.len()], 0, &self.joltages_target));

        loop {
            let State {mut joltages, n_push, diff_target } = queue.pop().unwrap();

            if  diff_target == 0 {
                return n_push;
            }

            for b in &self.buttons {
                b.apply_joltage(&mut joltages);
                let hash = cache.hasher().hash_one(&joltages);
                if !joltages.iter().zip(self.joltages_target.iter()).any(|(a, b)| a > b) {
                    if let Some(found) = cache.get_mut(&hash) {
                        if *found > n_push + 1 {
                            *found = n_push + 1;
                            queue.push(State::new(joltages.clone(), n_push + 1, &self.joltages_target));
                        }
                    } else {
                        cache.insert(hash, n_push + 1);
                        queue.push(State::new(joltages.clone(), n_push + 1, &self.joltages_target));
                    }
                }
                b.undo_joltage(&mut joltages);
            }
        }
    }
}
*/
