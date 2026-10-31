use core::cmp::Ordering;
use std::collections::HashMap;

fn decrypt_line(line: &str, num: u32) -> String {
    let mut res = String::new();
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '-' {
            if chars.peek().is_some_and(|c| c.is_ascii_digit()) {
                break
            }
            res.push(' ');
            continue;
        }

        let start = b'a' as u32;
        let end = b'z' as u32 - start + 1;
        let c = c as u32 - start;
        let c = ((c + num) % end) + start;
        res.push(char::try_from(c).unwrap());
    }
    res
}

fn solve(lines: &[impl AsRef<str>]) -> (usize, usize) {

    let mut pt1 = 0;
    let mut pt2 = 0;
    let mut set: HashMap<char, usize> = HashMap::new();
    for l in lines {
        let line = l.as_ref().split("-");
        set.clear();
        for chunk in line {
            if chunk.chars().next().unwrap().is_ascii_digit() {
                let (num, checksum) = chunk.split_once("[").unwrap();
                let num = num.parse::<usize>().unwrap();
                let checksum = checksum.strip_suffix("]").unwrap();

                let mut common: Vec<_> = set.iter().collect();
                common.sort_by(|a, b| {
                    match b.1.cmp(a.1) {
                        Ordering::Equal => a.0.cmp(b.0),
                        o => o
                    }
                });

                let is_match = common.iter().map(|a| a.0)
                    .zip(checksum.chars())
                    .all(|(found, expected)| *found == expected);

                if is_match {
                    pt1 += num;
                    let line = decrypt_line(l.as_ref(), num as _);
                    if line == "northpole object storage" {
                        pt2 = num;
                    }
                }
            } else {
                for c in chunk.chars() {
                    *set.entry(c).or_default() += 1;
                }
            }
        }
    }
    (pt1, pt2)
}

pub fn main() {
    let input: Vec<_> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use crate::{decrypt_line, solve};

    #[test]
    fn test() {
        let lines = &[
            "aaaaa-bbb-z-y-x-123[abxyz]",
            "a-b-c-d-e-f-g-h-987[abcde]",
            "not-a-real-room-404[oarel]",
            "totally-real-room-200[decoy]",
        ];
        assert_eq!(solve(lines).0, 1514);
        assert_eq!(decrypt_line("qzmt-zixmtkozy-ivhz-343", 343), "very encrypted name");
    }
}
