use std::collections::HashSet;

const ROTATIONS: [(i32,i32); 4] = [
    (-1,0), /* UP    */
    (0,1),  /* RIGHT */
    (1,0),  /* DOWN  */
    (0,-1), /* LEFT  */
];

fn get_next_pos(lines: &[Vec<char>], dir: (i32,i32), mut pos: (i32,i32)) -> Option<(i32,i32)> {
    pos.0 += dir.0;
    pos.1 += dir.1;

    let (y,x) = pos;

    if y < 0 || y as usize >= lines.len()
        || x < 0 || x as usize >= lines[y as usize].len()
    {
        None
    } else {
        Some(pos)
    }
}

fn find_loop(lines: &[Vec<char>], start: (i32,i32)) -> bool {
    let mut iter = ROTATIONS.iter().cycle();
    let mut dir = iter.next().unwrap();

    let mut collisions = HashSet::new();
    let mut pos = start;

    loop {
        let Some(newpos) = get_next_pos(lines, *dir, pos) else { return false };

        if lines[newpos.0 as usize][newpos.1 as usize] == '#' {
            if collisions.contains(&(pos,*dir)) {
                return true;
            }
            collisions.insert((pos,*dir));
            dir = iter.next().unwrap();
            continue;
        }

        pos = newpos;
    }
}

pub fn main() {
    let mut start: (i32,i32) = (0,0);

    let mut lines: Vec<Vec<char>> =
        aoc::get_input_file_lines()
            .enumerate()
            .map(|(y, mut l)| {
                if let Some(x) = l.find('^') {
                    start = (y as i32, x as i32);
                    l = l.replace('^', ".");
                }
                l.chars().collect()
            })
            .collect();

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut iter = ROTATIONS.iter().cycle();
    let mut direction = iter.next().unwrap();
    let mut pos = start;

    loop {
        let Some(nextpos) = get_next_pos(&lines, *direction, pos) else { break };

        if lines[nextpos.0 as usize][nextpos.1 as usize] == '#' {
            direction = iter.next().unwrap();
            continue;
        }

        pos = nextpos;
        visited.insert(pos);
    }

    let part1 = visited.len();
    println!("Part 1: {part1}");

    let mut part2 = 0;

    for (y,x) in visited {
        if (y,x) == start { continue }
        let y = y as usize;
        let x = x as usize;

        let c = lines[y][x];
        if c == '#' { continue }
        lines[y][x] = '#';

        if find_loop(&lines, start) {
            part2 += 1;
        }

        lines[y][x] = c;
    }

    println!("Part 2: {part2}");
}
