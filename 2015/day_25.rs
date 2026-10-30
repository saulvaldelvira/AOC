use std::collections::VecDeque;

fn solve(target_row: usize, target_col: usize) -> u64 {
    let mut code = 20151125;
    let mut queue = VecDeque::from([(1, 1)]);

    loop {
        let Some((row, col)) = queue.pop_front() else { unreachable!() };
        if row == target_row && col == target_col {
            return code;
        }
        if col == 1 {
            queue.push_back((row + 1, col));
        }
        queue.push_back((row, col + 1));
        code = (code * 252533) % 33554393;
    }
}


fn main() {
    let input = aoc::get_input_string();
    let input = input.strip_prefix("To continue, please consult the code grid in the manual.  Enter the code at row ").unwrap();
    let (row, column) = input.split_once(", column ").unwrap();
    let column = column.trim_end().strip_suffix(".").unwrap().parse().unwrap();
    let row = row.parse().unwrap();
    aoc::run(|| (solve(row, column), 0));
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let expected = [
            [20151125,18749137,17289845,30943339,10071777,33511524],
            [31916031,21629792,16929656, 7726640,15514188, 4041754],
            [16080970, 8057251, 1601130, 7981243,11661866,16474243],
            [24592653,32451966,21345942, 9380097,10600672,31527494],
            [77061,17552253,28094349, 6899651, 9250759,31663883],
            [33071741, 6796745,25397450,24659492, 1534922,27995004],
        ];

        for (row_idx, row) in expected.iter().enumerate() {
            for (col_idx, code) in row.iter().enumerate() {
                assert_eq!(solve(row_idx + 1, col_idx + 1), *code);
            }
        }
    }
}
