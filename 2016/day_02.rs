fn solve(map: &[&[char]], mut pos_x: i32, mut pos_y: i32, lines: &[impl AsRef<str>]) -> String {
    let mut res = String::new();

    for line in lines {
        for c in line.as_ref().trim().chars() {
            match c {
                'U' => pos_y -= 1,
                'L' => pos_x -= 1,
                'D' => pos_y += 1,
                'R' => pos_x += 1,
                _ => unreachable!()
            }
            pos_y = pos_y.min(map.len() as i32 - 1).max(0);
            pos_x = pos_x.min(map[pos_y as usize].len() as i32 - 1).max(0);
            if map[pos_y as usize][pos_x as usize] == 'X' {
                match c {
                    'U' => pos_y += 1,
                    'L' => pos_x += 1,
                    'D' => pos_y -= 1,
                    'R' => pos_x -= 1,
                    _ => unreachable!()
                }
            }
        }
        res.push(map[pos_y as usize][pos_x as usize]); 
    }

    res
}

const BUTTONS_PT1: &[&[char]] = &[
    &['1', '2', '3'],
    &['4', '5', '6'],
    &['7', '8', '9'],
];
const BUTTONS_PT2: &[&[char]] = &[
    &['X', 'X', '1', 'X', 'X'],
    &['X', '2', '3', '4', 'X'],
    &['5', '6', '7', '8', '9'],
    &['X', 'A', 'B', 'C', 'X'],
    &['X', 'X', 'D', 'X', 'X'],
];

pub fn main() {
    let input: Vec<_> = aoc::get_input_file_lines().collect();
    aoc::run(|| (solve(BUTTONS_PT1, 1, 1, &input), solve(BUTTONS_PT2, 2, 0, &input)));
}

#[cfg(test)]
mod test {
    use crate::{solve, BUTTONS_PT1, BUTTONS_PT2};

    #[test]
    fn test() {
        let input = "ULL
RRDDD
LURDL
UUUUD".lines().collect::<Vec<_>>();
        assert_eq!(solve(BUTTONS_PT1, 1, 1, &input), "1985");
        assert_eq!(solve(BUTTONS_PT2, 0, 2, &input), "5DB3");
    }
}