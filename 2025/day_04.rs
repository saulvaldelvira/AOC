use aoc::get_input_matrix;

fn get_removable(input: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize)> {
    (0..input.len())
    .flat_map(|row| (0..input[row].len()).map(move |col| (row, col)))
    .filter(|&(row, col)| {
        const DIRECTIONS: &[(isize, isize)] = &[
            (-1, 1) ,(0, 1), (1, 1),
            (-1, 0) ,        (1, 0),
            (-1, -1) ,(0, -1), (1, -1),
        ];

        if input[row][col] != b'@' { return false }

        let sum: usize = DIRECTIONS.iter()
            .filter_map(|&(x, y)| {
                let r = row.checked_add_signed(y)?;
                let c  = col.checked_add_signed(x)?;
                if r >= input.len() || c >= input[r].len() { return None }

                if input[r][c] == b'@' { Some(1) } else { None }
            })
        .sum();

        sum < 4
    })
}

fn solve(input: &mut [Vec<u8>]) -> (usize, usize) {
    let part1 = get_removable(input).count();

    let mut part2 = 0;

    loop {
        let pos: Vec<_> = get_removable(input).collect();
        if pos.is_empty() { break }
        for (row, col) in pos {
            part2 += 1;
            input[row][col] = b'.';
        }
    }

    (part1, part2)
}

fn main() {
    let mut input = get_input_matrix();
    aoc::run(|| solve(&mut input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
    let (part1, part2) = super::solve(
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
.lines()
.map(|l| l.as_bytes().to_vec())
.collect::<Vec<_>>().as_mut_slice()
    );
        assert_eq!(part1, 13);
        assert_eq!(part2, 43);
    }
}
