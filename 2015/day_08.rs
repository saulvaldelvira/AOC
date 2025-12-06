use aoc::get_input_file_lines;

fn escaped_len(s: &str) -> usize {
    let mut chars = s.chars();
    let mut count = 0;
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next().unwrap() {
                '\\' |
                '"'  => {}
                'x' => {
                    chars.next();
                    chars.next();
                }
                _ => unreachable!()
            }
        }
        count += 1;
    }

    count
}

fn encoded_len(s: &str) -> usize {
    s.chars().map(|c| {
        match c {
            '"' => 2,
            '\\' => 2,
            _ => 1,
        }
    })
    .sum()
}

fn solve<I, S>(lines: I) -> (usize, usize)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        let line = line.as_ref();

        let code_len = line.len();
        let actual_len = escaped_len(line) - 2;
        let encoded_len = encoded_len(line) + 2;

        part1 += code_len - actual_len;
        part2 += encoded_len - code_len;
    }

    (part1, part2)
}

fn main() {
    let input = get_input_file_lines();
    aoc::run(|| solve(input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let (part1, part2) = super::solve(
            [
                "",
                "abc",
                "aaa\\\"aaa",
                "\\x27"
            ]
            .iter()
            .map(|s| format!("\"{s}\""))
        );

        assert_eq!(part1, 12);
        assert_eq!(part2, 19);
    }
}
