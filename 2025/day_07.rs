use std::collections::HashMap;

use aoc::get_input_matrix;

fn count_splits(start: usize, mut lines: Vec<Vec<u8>>) -> usize {
    let mut beams = vec![start];

    let mut n_splits = 0;
    for line in lines.iter_mut().skip(1) {
        let mut new_beams = Vec::new();
        for &beam in &beams {
            if line[beam] == b'^' {
                let mut is_split = false;
                if beam > 0 && line[beam - 1] == b'.' {
                    line[beam - 1] = b'|';
                    is_split = true;
                    new_beams.push(beam - 1);
                }
                if beam < line.len() - 1 && line[beam + 1] == b'.' {
                    line[beam + 1] = b'|';
                    is_split = true;
                    new_beams.push(beam + 1);
                }
                if is_split {
                    n_splits += 1;
                }
            } else {
                line[beam] = b'|';
                new_beams.push(beam);
            }
        }
        beams = new_beams;
    }

    n_splits
}

fn count_timelines(
    beam: usize,
    row: usize,
    cache: &mut HashMap<(usize, usize), usize>,
    lines: &[Vec<u8>]
) -> usize
{
    if row == lines.len() { return 1 }
    if let Some(count) = cache.get(&(row, beam)) {
        return *count
    }

    let count = if lines[row][beam] == b'^' {
        let mut result = 0;
        if beam > 0 {
            result += count_timelines(beam - 1, row, cache, lines);
        }
        if beam < lines[0].len() - 1 {
            result += count_timelines(beam + 1, row, cache, lines);
        }
        result
    } else {
        count_timelines(beam, row + 1, cache, lines)
    };

    cache.insert((row, beam), count);
    count
}

fn solve(lines: Vec<Vec<u8>>) -> (usize, usize) {
    let start = lines[0].iter().enumerate().find(|&(_, &c)| c == b'S').unwrap().0;

    let part2 = count_timelines(start, 1, &mut HashMap::new(), &lines);
    // Part1 mutates 'lines', so call it after part2
    let part1 = count_splits(start, lines);

    (part1, part2)
}

fn main() {
    let input: Vec<Vec<u8>> = get_input_matrix();
    aoc::run(|| solve(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let (part1, part2) = super::solve("\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
.lines()
.map(|s| s.as_bytes().to_vec())
.collect::<Vec<_>>());
        assert_eq!(part1, 21);
        assert_eq!(part2, 40);
    }
}
