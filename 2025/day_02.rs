use aoc::get_input_string;

fn solve(input: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    for range in input.split(',') {
        let (low, high) = range.split_once('-').expect("Expected this format <low>-<hi>");
        let (low, high) = (
            low.parse::<usize>().unwrap(),
            high.parse::<usize>().unwrap(),
        );

        fn check(mut num: usize, ndigits: u32, n_chunks: u32) -> bool {
            if !ndigits.is_multiple_of(n_chunks) { return false }

            let chunk_len = ndigits / n_chunks;
            let div = 10_usize.pow(chunk_len);

            let mut i = 1;
            loop {
                let chunk = num % div;
                num /= div;

                if chunk != (num % div) { return false }

                i += 1;
                if i == n_chunks { return true }
            }
        }

        for i in low..=high {
            let ndigits = i.checked_ilog10().unwrap_or(0) + 1;
            if check(i, ndigits, 2) {
                part1 += i;
                part2 += i;
            } else {
                for p in 3..=ndigits {
                    if check(i, ndigits, p) {
                        part2 += i;
                        break
                    }
                }
            }
        }

    }

    (part1, part2)
}

fn main() {
    let input = get_input_string();
    aoc::run(|| solve(input.trim()));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
    let (part1, part2) = super::solve("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
        assert_eq!(part1, 1227775554);
        assert_eq!(part2, 4174379265);
    }
}
