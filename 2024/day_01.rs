
pub fn main() {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    aoc::get_input_file_lines()
    .for_each(|line| {
        let mut it = line.split_whitespace();
        let n1: i32 = it.next().unwrap().parse().unwrap();
        let n2: i32 = it.next().unwrap().parse().unwrap();
        list1.push(n1);
        list2.push(n2);
    });

    list1.sort();
    list2.sort();

    let part1: i32 = list1.iter()
        .zip(list2.iter())
        .map(|n| (n.0 - n.1).abs())
        .sum();

    println!("Part1: {part1}");

    let part2: i32 = list1.iter().map(|&n| {
        let count = list2.iter().filter(|&&n2| n2 == n).count();
        n * count as i32
    }).sum();

    println!("Part2: {part2}");
}
