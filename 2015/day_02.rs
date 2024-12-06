
pub fn main() {
    let (part_1,part_2): (usize,usize) =
    aoc::get_input_file_lines()
    .map(|line| {
        let mut line = line.split('x');
        let length: usize = line.next().unwrap().parse().unwrap();
        let width: usize  = line.next().unwrap().parse().unwrap();
        let height: usize = line.next().unwrap().parse().unwrap();

        let lw = length * width;
        let wh = width * height;
        let hl = height * length;

        let mut sorted = [length,width,height];
        sorted.sort();
        let [shortest1,shortest2,..] = sorted;

        let part1 = 2 * lw + 2 * wh + 2 * hl + shortest1;
        let part2 = length * height * width + shortest1 * 2 + shortest2 * 2;

        (part1,part2)
    }).fold((0,0), |acc, val| {
        (acc.0 + val.0, acc.1 + val.1)
    });

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
