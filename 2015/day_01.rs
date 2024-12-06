use std::io::Read;

pub fn main() {
    let mut part_2 = None;
    let part_1: i32 =
        aoc::get_input_file()
        .bytes()
        .map_while(Result::ok)
        .enumerate()
        .fold(0, |acc, (i,e)| {
            let acc = match e {
                b'(' => acc + 1,
                b')' => acc - 1,
                _ => acc
            };
            if acc < 0 && part_2.is_none() {
                part_2 = Some(i + 1);
            };
            acc
        });

    println!("Part 1: {part_1}");
    println!("Part 2: {}", part_2.unwrap_or(0));
}
