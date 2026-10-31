use std::collections::HashSet;

fn solve(lines: &[impl AsRef<str>]) -> (usize, usize) {
    let mut pt1 = 0;
    let mut pt2 = 0;

    let mut abas = HashSet::new();

    for line in lines.iter().map(AsRef::as_ref) {
        let mut abba_outside = false;
        let mut abba_inside = false;

        let mut nest_level = 0;
        fn check_nest(nl: &mut usize, c: u8) -> bool {
            if c == b'[' {
                *nl += 1;
                true
            } else if c == b']' {
                *nl -= 1;
                true
            } else { false }
        }

        /* Part 1 */
        for chunk in line.as_bytes().array_windows::<4>() {
            let [a, b, c, d] = chunk;
            if check_nest(&mut nest_level, *a) { continue }

            if a != b && c == b && d == a {
                if nest_level > 0 {
                    abba_inside = true;
                    break;
                }
                else { abba_outside = true; }
            }
        }

        if abba_outside && !abba_inside {
            pt1 += 1;
        }

        /* Part 2 */
        nest_level = 0;
        abas.clear();
        for chunk in line.as_bytes().array_windows::<3>() {
            let [a, b, c] = chunk;
            if check_nest(&mut nest_level, *a) { continue }

            if nest_level == 0 && a != b && a == c {
                abas.insert((a, b));
            }
        }

        if abas.is_empty() { continue }

        nest_level = 0;
        for chunk in line.as_bytes().array_windows::<3>() {
            let [a, b, c] = chunk;
            if check_nest(&mut nest_level, *a) { continue }

            if nest_level > 0 && a != b && a == c && abas.contains(&(b, a)) {
                pt2 += 1;
                break
            }
        }
    }

    (pt1, pt2)
}

pub fn main() {
    let input: Vec<_> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn test() {
        let (pt1, _) = solve("\
abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn
".lines().collect::<Vec<_>>().as_slice());
        assert_eq!(pt1, 2);

        let (_, pt2) = solve("\
aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb
".lines().collect::<Vec<_>>().as_slice());
        assert_eq!(pt2, 3);
    }
}
