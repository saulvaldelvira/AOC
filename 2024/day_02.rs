use std::{fs::File, io::{BufRead, BufReader}};

fn is_safe(line: &[i32], skip: Option<usize>) -> bool {
    let mut first = &line[..];
    let mut second: &[i32] = &[];

    if let Some(n) = skip {
        first = &line[..n];
        second = &line[n+1..];
    }

    let it = first.iter().chain(second.iter());

    let n1 = it.clone().next().unwrap();
    let n2 = it.clone().skip(1).next().unwrap();
    let increase = n2 > n1;

    it.clone()
    .zip(it.skip(1))
    .all(|(&a,&b)| {
        if increase && b < a || !increase && b > a {
            false
        } else {
            let diff = (b - a).abs();
            diff >= 1 && diff <= 3
        }
    })
}

fn main() {
    let f = File::open("input.txt").expect("Error opening file");
    let f = BufReader::new(f);

    let lines: Vec<Box<[i32]>> =
        f.lines().map_while(Result::ok).map(|line| {
            let line: Vec<i32> =
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
            line.into_boxed_slice()
        })
        .collect();

    let part1 =
        lines.iter()
        .filter(|line| is_safe(line, None))
        .count();

    println!("Part 1: {part1}");

    let part2 = lines.iter().filter(|line| {
        is_safe(line, None)
        || (0..line.len()).any(|n| is_safe(line, Some(n)))
    }).count();

    println!("Part 2: {part2}");
}
