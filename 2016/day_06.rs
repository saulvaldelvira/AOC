use std::collections::HashMap;

fn solve(lines: &[impl AsRef<str>]) -> (String, String) {
    let len = lines[0].as_ref().len();
    let mut freqs = vec![HashMap::<char, usize>::new(); len];

    for line in lines {
        for (c, map) in line.as_ref().chars().zip(freqs.iter_mut()) {
            *map.entry(c).or_default() += 1;
        }
    }

    let pt1 = freqs.iter().map(|freq| {
        *freq.iter().max_by_key(|(_, s)| *s).unwrap().0
    })
    .collect();

    let pt2 = freqs.iter().map(|freq| {
        *freq.iter().min_by_key(|(_, s)| *s).unwrap().0
    })
    .collect();

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
        let (pt1, pt2) = solve("\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
".lines().collect::<Vec<_>>().as_slice());
        assert_eq!(pt1, "easter");
        assert_eq!(pt2, "advent");
    }
}
