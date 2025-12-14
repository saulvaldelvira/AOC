
fn solve(input: &[String], n_secs: u16) -> (usize, usize) {
    struct Reindeer {
        velocity: u16,
        secs_fly: u16,
        rest_time: u16,

        points: u16,
        curr_fly: u16,
        curr_sleep: u16,
        kms: u16,
    }

    impl Reindeer {
        fn iterate(&mut self) {
            if self.curr_fly == self.secs_fly {
                self.curr_sleep += 1;
                if self.curr_sleep == self.rest_time {
                    self.curr_sleep = 0;
                    self.curr_fly = 0;
                }
            } else {
                self.kms += self.velocity;
                self.curr_fly += 1;
            }
        }
    }

    let mut reindeers: Vec<_> =
        input.iter()
        .map(|line| {
            let (_, rest) = line.split_once(" can fly ").unwrap();

            let (veltime, rest) = rest.split_once(", but then must rest for ").unwrap();

            let (vel, secs) = veltime.split_once(" km/s for ").unwrap();
            let secs = secs.strip_suffix(" seconds").unwrap();

            let velocity = vel.parse::<u16>().unwrap();
            let secs_fly = secs.parse::<u16>().unwrap();

            let rest_time = rest.strip_suffix(" seconds.")
                .unwrap().parse::<u16>().unwrap();

            Reindeer {
                velocity, secs_fly, rest_time,
                curr_fly: 0, curr_sleep: 0, points: 0, kms: 0,
            }
        })
        .collect();

    for _ in 0..n_secs {
        let max_kms = reindeers
            .iter_mut()
            .map(|r| {
                r.iterate();
                r.kms
            })
            .max().unwrap();

        for r in reindeers.iter_mut() {
            if r.kms == max_kms {
                r.points += 1;
            }
        }
    }

    let part1 = reindeers.iter().map(|r| r.kms).max().unwrap();
    let part2 = reindeers.iter().map(|r| r.points).max().unwrap();

    (part1 as _, part2 as _)
}

fn main() {
    let input: Vec<String> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve(&input, 2503));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let (part1, part2) = super::solve("\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
.lines()
.map(str::to_string)
.collect::<Vec<_>>()
.as_slice(), 1000);

        assert_eq!(part1, 1120);
        assert_eq!(part2, 689);
    }
}
