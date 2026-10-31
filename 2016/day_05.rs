use core::fmt::Write;

use aoc::dprintln;


pub fn solve(input: &str) -> (String, String) {
    let input_len = input.len();
    let mut input = String::from(input);
    let mut result = String::new();

    let mut idx = 0;

    let mut pt2 = [b'_'; 8];
    let mut pt2_len = 0;

    while result.len() < 8 || pt2_len < 8 {
        input.truncate(input_len);
        input.write_fmt(format_args!("{idx}")).unwrap();
        let hash = md5::compute(input.as_bytes());
        if hash[0] == 0x00 && hash[1] == 0x00 && hash[2] & 0xF0 == 0 {
            let c = char::from_digit(hash[2] as u32 & 0x0F, 16).unwrap();
            if result.len() < 8 {
                result.push(c);
            }
            let pos = c as usize - b'0' as usize;
            if (0..8).contains(&pos) && pt2[pos] == b'_' {
                let c = char::from_digit(hash[3] as u32 >> 4, 16).unwrap();
                pt2[pos] = c as u8;
                pt2_len += 1;
            }
            dprintln!("{input} => {hash:X}\n PT1: {result}\n PT2: {}", str::from_utf8(&pt2).unwrap());
        }
        idx += 1;
    }
    (result, str::from_utf8(&pt2).unwrap().to_string())
}

pub fn main() {
    let input = std::env::args().nth(1).unwrap();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn test() {
        let (pt1, pt2) = solve("abc");
        assert_eq!(pt1, "18f47a30");
        assert_eq!(pt2, "05ace8e3");
    }
}
