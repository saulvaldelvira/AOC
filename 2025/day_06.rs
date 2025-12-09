use aoc::get_input_lines;
use aoc::num_digits;

fn solve(lines: &[String]) -> (usize, usize) {

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Alignment {
        Left,
        Right,
        None,
    }

    let mut ops: Vec<_> =
        lines.last().unwrap()
        .char_indices()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(i, op)| {
            (i, op, Alignment::None)
        }).collect();
    // Fake column
    ops.push((lines[0].len() + 2, '?', Alignment::None));

    let mut columns: Vec<Vec<usize>> = Vec::with_capacity(ops.len());

    for idx in 0..ops.len() - 1 {
        let next = ops[idx + 1];
        let op = &mut ops[idx];
        let mut digits = Vec::with_capacity(lines.len() - 1);

        for line in lines.iter().take(lines.len() - 1) {
            let mut offset = op.0;

            if line[offset..].starts_with(' ') {
                op.2 = Alignment::Right;
                offset += line[offset..].find(|c: char| c.is_ascii_digit()).unwrap();
            }

            if op.2 == Alignment::None {
                let num_finish = line[offset..]
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .unwrap().0 + 2 + offset;
                let next_offset = next.0;
                if num_finish < next_offset {
                    op.2 = Alignment::Left;
                }
            }

            let slice = line[offset..].split_whitespace().next().unwrap();
            let digit = slice.trim().parse::<usize>().unwrap();
            digits.push(digit);
        }
        columns.push(digits);
    }

    let part1 = columns.iter().zip(ops.iter()).map(|(col, op)| {
        let init = col[0];
        col.iter().skip(1).fold(init, |acc, elem| {
            match op.1 {
                '+' => acc + elem,
                '-' => acc - elem,
                '*' => acc * elem,
                '/' => acc / elem,
                _ => unreachable!()
            }
        })
    }).sum::<usize>();

    fn get_num(alignment: Alignment, nums: &mut [usize]) -> usize {
        let mut acc = 0;
        let longest = nums.iter().map(|n| num_digits!(n)).max().unwrap();
        for num in nums.iter_mut() {
            if *num == 0 { continue }
            if num_digits!(num) < longest && alignment == Alignment::Left { continue }
            acc *= 10;
            acc += *num % 10;
            *num /= 10;
        }

        acc
    }

    let part2 = columns.iter_mut().zip(ops.iter()).map(|(col, op)| {
        let longest = col.iter().map(|n| num_digits!(n)).max().unwrap();
        let mut init = get_num(op.2, col);
        for _ in 1..longest{
            let n = get_num(op.2, col);

            init = match op.1 {
                '+' => init + n,
                '-' => init - n,
                '*' => init * n,
                '/' => init / n,
                _ => unreachable!()
            };
        }

        init
    }).sum::<usize>();

    (part1, part2)
}

fn main() {
    let input: Vec<String> = get_input_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let (part1, part2) = super::solve(&[
            String::from("123 328  51 64 "),
            String::from(" 45 64  387 23 "),
            String::from("  6 98  215 314"),
            String::from("*   +   *   +"),
        ]);
        assert_eq!(part1, 4277556);
        assert_eq!(part2, 3263827);
    }
}
