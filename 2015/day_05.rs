use fancy_regex::Regex;

pub fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    let part1_regex: Vec<_> = [
        "([aeiou].*){3,}",
        r#"(.)\1"#,
        "(ab|cd|pq|xy)",
    ].into_iter().map(|r| {
        Regex::new(r).unwrap()
    }).collect();

    let part2_regex: Vec<_> = [
        r"(..).*\1",
        r"(.).\1",
    ].into_iter().map(|r| {
        Regex::new(r).unwrap()
    }).collect();

    for line in aoc::get_input_file_lines() {
        if part1_regex[0].is_match(&line).unwrap()
            && part1_regex[1].is_match(&line).unwrap()
            && !part1_regex[2].is_match(&line).unwrap()
        {
                part1 += 1;
        }

        if part2_regex[0].is_match(&line).unwrap()
            && part2_regex[1].is_match(&line).unwrap()
        {
            part2 += 1;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
