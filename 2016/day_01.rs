use std::collections::HashSet;

fn solve(dirs: &[&str]) -> (i32, i32) {
    let mut dir: i32 = 0;
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;

    let mut seen = HashSet::new();
    let mut pt2 = None;

    for turn in dirs {
        let (direction, num) = turn.trim().split_at(1);
        let num: i32 = num.parse().unwrap();
        dir = match direction {
            "R" => (dir + 90) % 360,
            "L" => if dir >= 90 { dir - 90 } else { 270 },
            _ => unreachable!()
        };
        for _ in 0..num {
            match dir {
                0 => pos_y += 1,
                90 => pos_x += 1,
                180 => pos_y -= 1,
                270 => pos_x -= 1,
                _ => unreachable!()
            }
            if pt2.is_none() && !seen.insert((pos_x, pos_y)) {
                pt2 = Some(pos_x.abs() + pos_y.abs());
            }
        }
    }

    (pos_x.abs() + pos_y.abs(), pt2.unwrap_or(-1))
}

pub fn main() {
    let input = aoc::get_input_string();
    let dirs: Vec<_> = input.split(", ").collect();
    aoc::run(|| solve(&dirs));
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn test() {
        assert_eq!(solve(&["R2", "L3"]).0, 5);
        assert_eq!(solve(&["R2", "R2", "R2"]).0, 2);
        assert_eq!(solve(&["R5", "L5", "R5", "R3"]).0, 12);
        assert_eq!(solve(&["R8", "R4", "R4", "R8"]).1, 4);
    }
}
