use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::BufRead;

pub fn main() {
    let mut rules: HashSet<(i32, i32)> = HashSet::new();

    let mut lines = aoc::get_input_file()
                        .lines()
                        .map_while(Result::ok);

    for line in lines.by_ref() {
        if line.trim().is_empty() { break }

        let mut line = line.trim().split('|');
        let bef: i32 = line.next().unwrap().parse().unwrap();
        let aft: i32 = line.next().unwrap().parse().unwrap();

        rules.insert((bef,aft));
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for line in lines {
        let mut page: Vec<i32> =
                line.split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();

        if page.is_sorted_by(|&a,&b|
                             !rules.contains(&(b,a)))
        {
            part1 += page[page.len() / 2];
        } else {
            page.sort_by(|&a,&b| {
                if rules.contains(&(a,b)) {
                    Ordering::Less
                } else if rules.contains(&(b,a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            part2 += page[page.len() / 2];
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
