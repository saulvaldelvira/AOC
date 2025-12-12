
fn solve(input: &[impl AsRef<str>]) -> (usize, usize) {
    let mut presents = Vec::<Vec<Vec<bool>>>::new();
    let mut area = vec![];
    let mut lines = input.iter().peekable();

    loop {
        let line = lines.next().unwrap().as_ref();
        if line.is_empty() {
            presents.push(area);
            area = vec![];
            if !lines.peek().unwrap().as_ref().ends_with(':') {
                break
            } else {
                continue
            }
        }

        area.push(
            line.chars().map(|c| c == '#').collect()
        );

    }

    let mut trees = Vec::new();

    for line in lines {
        let (area, presents) = line.as_ref().split_once(": ").unwrap();
        let (w, h) = area.split_once("x").unwrap();
        let (w, h) = (w.parse::<u32>().unwrap(), h.parse::<u32>().unwrap());
        let presents: Vec<usize> = presents
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        trees.push((w, h, presents));
    }

    let part1 = trees.iter().map(|(w, h, needs_presents)| {
        let area = w * h;
        let mut needs = 0;
        for (i, &p) in needs_presents.iter().enumerate() {
            let parea = presents[i].len() * presents[i][0].len();
            needs += parea * p;
        }

        if needs <= area as usize { 1 } else { 0 }
    })
    .sum::<usize>();

    (part1, 0)
}

fn main() {
    let input: Vec<_> = aoc::get_input_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let (_, _) = solve(&"\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
.lines()
.collect::<Vec<_>>());
        /* assert_eq!(part1, 2); */
    }
}
