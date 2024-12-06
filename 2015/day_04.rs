
pub fn main() {
    let secret_key = "yzbqklnj";

    let mut part1 = None;
    let mut part2 = 0;

    loop {
        let buf = format!("{secret_key}{part2}");
        let hash = md5::compute(buf.as_bytes());
        let hash = format!("{hash:x}");
        if hash.starts_with("000000") {
            break
        } else if hash.starts_with("00000") && part1.is_none() {
            part1 = Some(part2);
        }
        part2 += 1;
    }

    println!("Part 1: {}", part1.unwrap_or(0));
    println!("Part 2: {part2}");
}
