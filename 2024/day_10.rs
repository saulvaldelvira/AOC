use std::collections::HashSet;

type Point = aoc::point::Point<i32>;

fn find_score(p: Point, map: &[Box<[u8]>]) -> (i32,i32) {
    fn is_valid_point(p: Point, map: &[Box<[u8]>], level: u8) -> bool {
        const UP: Point = Point { x: 0, y: 0 };
        let down = Point {
            y: map.len() as i32 - 1,
            x: map[0].len() as i32 - 1,
        };

        if !p.is_withing(&UP, &down) {
            return false
        }

        if map[p.y as usize][p.x as usize] != level {
            return false;
        }

        true
    }
    fn find_possible_targets(p: Point, map: &[Box<[u8]>], level: u8) -> Option<Vec<Point>> {
        if !is_valid_point(p, map, level) {
            return None;
        }

        if level ==  b'9' {
            return Some(vec![p])
        }

        let points = [
            p + (0,1),
            p + (0,-1),
            p + (1,0),
            p + (-1,0),
        ];

        let mut founds = HashSet::new();
        for point in points {
            if let Some(p) = find_possible_targets(point, map, level + 1) {
                for p in p {
                    founds.insert(p);
                }
            }
        }

        if !founds.is_empty() {
            Some(founds.into_iter().collect::<Vec<_>>())
        } else {
            None
        }
    }
    fn find_number_of_paths(p: Point, map: &[Box<[u8]>], level: u8) -> i32 {
        if !is_valid_point(p, map, level) {
            return 0
        }

        if level ==  b'9' {
            return 1;
        }

        let points = [
            p + (0,1),
            p + (0,-1),
            p + (1,0),
            p + (-1,0),
        ];

        let mut count = 0;
        for point in points {
            count += find_number_of_paths(point, map, level + 1);
        }
        count

    }

    let pt1 = find_possible_targets(p, map, b'0').map(|v| v.len() as i32).unwrap_or(0);
    let pt2 = find_number_of_paths(p, map, b'0');
    (pt1,pt2)
}

fn solve(src: &str) -> (i32,i32) {
    let map = aoc::input_to_byte_array(src);
    let mut part1 = 0;
    let mut part2 = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == b'0' {
                let p = Point {
                    y: y as i32,
                    x: x as i32,
                };
                let (p1,p2) = find_score(p, &map);
                part1 += p1;
                part2 += p2;
            }
        }
    }
    (part1,part2)
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let (part1,part2) = solve("\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732");

        assert_eq!(part1, 36);
        assert_eq!(part2, 81);
    }
}
