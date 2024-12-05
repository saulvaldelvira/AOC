use std::fs;
use regexpr::Regex;

fn unwrap_mul(src: &str) -> (i32,i32) {
    let src = src.strip_prefix("mul(").unwrap();
    let src = src.strip_suffix(")").unwrap();
    let mut src = src.split(',');
    let left: i32 = src.next().unwrap().parse().unwrap();
    let right: i32 = src.next().unwrap().parse().unwrap();
    (left,right)
}

pub fn main() {
    let file = fs::read_to_string("input.txt")
                  .expect("Couldn't find input.txt");

    const MUL_REGEX: &str = r#"mul\([0-9]{1,3},[0-9]{1,3}\)"#;

    /***** PART 1 *****/
    let regex = Regex::compile(MUL_REGEX).unwrap();
    let part1: i32 =
    regex.find_matches(&file).map(|m| {
        let (l,r) = unwrap_mul(m.get_slice());
        l * r
    }).sum();
    println!("Part 1: {part1}");

    /***** PART 2 *****/
    let part2_regex = format!("{MUL_REGEX}|do(n't)?");
    let regex = Regex::compile(&part2_regex).unwrap();
    let mut enabled = true;
    let part2: i32 =
    regex.find_matches(&file).map(|m| {
        if m.get_slice().starts_with("do") {
           enabled = !m.get_slice().starts_with("don't");
           0
        } else if enabled {
            let (l,r) = unwrap_mul(m.get_slice());
            l * r
        } else { 0 }
    }).sum();
    println!("Part 2: {part2}");
}
