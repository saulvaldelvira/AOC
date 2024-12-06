use std::collections::HashMap;

fn move_coords(coords: &mut (i32,i32), dir: u8) -> bool {
    match dir {
        b'^' => coords.0 += 1,
        b'v' => coords.0 -= 1,
        b'>' => coords.1 += 1,
        b'<' => coords.1 -= 1,
        _ => return false
    }
    true
}

pub fn main() {
    let mut coords = (0_i32,0_i32);
    let mut presents = HashMap::new();

    for dir in aoc::get_input_file_bytes() {
        if move_coords(&mut coords, dir) {
            *presents.entry(coords).or_insert(0) += 1;
        }
    }

    let part1 = presents.len();
    println!("Part 1: {part1}");

    presents.clear();

    let mut santa_coords = (0_i32,0_i32);
    let mut robo_coords = (0_i32,0_i32);

    let mut turn_santa = true;

    for dir in aoc::get_input_file_bytes() {
        let coords = if turn_santa {
            &mut santa_coords
        } else {
            &mut robo_coords
        };
        if move_coords(coords, dir) {
            *presents.entry(*coords).or_insert(0) += 1;
        }
        turn_santa = !turn_santa;
    }

    let part2 = presents.len();
    println!("Part 2: {part2}");
}
