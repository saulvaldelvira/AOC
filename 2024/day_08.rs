use std::collections::{HashMap, HashSet};
use aoc::point::Point;

fn solve(src: &str) -> (Vec<Point<i32>>,Vec<Point<i32>>) {
    let width = src.lines().next().unwrap().len() as i32;
    let height = src.lines().filter(|l| !l.is_empty()).count() as i32;

    let mut map: HashMap<char, Vec<Point<i32>>> = HashMap::new();
    let mut points = HashSet::new();

    for (y,line) in src.lines()
                       .filter(|l| !l.is_empty())
                       .enumerate()
    {
        for (x,c) in line.chars().enumerate() {
            if c == '.' { continue }
            let p = Point {
                x: x as i32,
                y: y as i32,
            };
            map.entry(c).or_default().push(p);
            points.insert(p);
        }
    }

    let mut antinodes_pt1 = HashSet::new();
    let mut antinodes_pt2 = HashSet::new();

    let upbound = Point{x: 0, y: 0};
    let downbound = Point{y: height - 1, x: width - 1};

    for v in map.values() {
        for &a in v.iter() {
            for &b in v.iter().filter(|&&b| b != a) {
                let earring = a.get_earring(&b);

                let (max,min) = (a.max(b), a.min(b));

                let antinode1 = min - earring;
                let antinode2 = max + earring;

                for an in [antinode1, antinode2] {
                    if an.is_withing(&upbound, &downbound) {
                        antinodes_pt1.insert(an);
                    }
                }

                let mut curr = min;
                while curr.is_withing(&upbound, &downbound) {
                    antinodes_pt2.insert(curr);
                    curr -= earring;
                }

                let mut curr = max;
                while curr.is_withing(&upbound, &downbound) {
                    antinodes_pt2.insert(curr);
                    curr += earring;
                }
            }
        }
    }

    (
        antinodes_pt1.into_iter().collect(),
        antinodes_pt2.into_iter().collect()
    )
}

pub fn main() {
    let (part1,part2) = solve(include_str!("input.txt"));

    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
    let (part1,part2) = solve("\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............");
        assert_eq!(part1.len(), 14);
        assert_eq!(part2.len(), 34);
    }
}
