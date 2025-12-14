use core::mem;


fn iter(input: &[Vec<bool>], out: &mut [Vec<bool>], part2: bool) {
    let corners = [
        (0, 0),
        (0, input[0].len() - 1),
        (input.len() - 1, 0),
        (input.len() - 1, input[0].len() - 1)
    ];
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            out[row][col] = input[row][col];

            if part2 && corners.contains(&(row, col)) {
                continue;
            }

            const DIRECTIONS: &[(isize, isize)] = &[
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),  (1, 0),   (1, 1)
            ];

            let mut neighbors_on = 0;
            for (drow, dcol) in DIRECTIONS {
                let Some(nrow) = row.checked_add_signed(*drow) else { continue };
                let Some(ncol) = col.checked_add_signed(*dcol) else { continue };
                if nrow >= input.len() || ncol >= input[nrow].len() { continue }


                if input[nrow][ncol] {
                    neighbors_on += 1;
                }
            }

            if input[row][col] {
                if !(2..=3).contains(&neighbors_on) {
                    out[row][col] = false;
                }
            } else if neighbors_on == 3 {
                out[row][col] = true;
            }

        }
    }
}

fn solve(
    input: Vec<Vec<bool>>,
    n_steps_p1: usize,
    n_steps_p2: usize
) -> (usize, usize)
{
    let mut buf = input.clone();
    let mut part1_state = input.clone();
    let mut part2_state = input;

    let f = part2_state.first_mut().unwrap();
    f[0] = true;
    *f.last_mut().unwrap() = true;
    let l = part2_state.last_mut().unwrap();
    l[0] = true;
    *l.last_mut().unwrap() = true;

    let mut apply = |state: &mut Vec<Vec<bool>>, pt2: bool| {
        iter(state, &mut buf, pt2);
        mem::swap(state, &mut buf);
    };

    for _ in 0..n_steps_p1 {
        apply(&mut part1_state, false);
        apply(&mut part2_state, true);
    }
    for _ in n_steps_p1..n_steps_p2 {
        apply(&mut part2_state, true);
    }

    fn count_on(m: &[Vec<bool>]) -> usize {
        m.iter()
            .map(|line| line.iter().filter(|&&on| on).count())
            .sum()
    }

    let part1 = count_on(&part1_state);
    let part2 = count_on(&part2_state);


    (part1, part2)
}

fn main() {
    let input: Vec<Vec<bool>> = aoc::get_input_lines_mapped(|b| b == b'#');
    aoc::run(|| solve(input, 100, 100));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let input = "\
.#.#.#
...##.
#....#
..#...
#.#..#
####.."
.lines()
.map(|l| l.bytes().map(|c| c == b'#').collect())
.collect();

        let (part1, part2) = super::solve(input, 4, 5);
        assert_eq!(part1, 4);
        assert_eq!(part2, 17);

    }
}
