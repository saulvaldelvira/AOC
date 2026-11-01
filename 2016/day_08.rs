fn draw_screen(screen: &[impl AsRef<[bool]>]) {
    for row in screen.iter() {
        for col in row.as_ref().iter() {
            print!("{}", if *col { '#' } else { '.' });
        }
        println!();
    }
}

fn solve(lines: &[impl AsRef<str>]) -> usize {
    let mut display = [[false; 50]; 6];
    let mut display_tmp: [[bool; 50]; 6];

    for line in lines.iter().map(AsRef::as_ref) {
        let (cmd, args) = line.split_once(" ").unwrap();
        println!("{line}");
        match cmd {
            "rect" => {
                let (width, height) = args.split_once("x").unwrap();
                let width = width.parse::<usize>().unwrap();
                let height = height.parse::<usize>().unwrap();
                for row in display.iter_mut().take(height) {
                    for col in row.iter_mut().take(width) {
                        *col = true;
                    }
                }
            }
            "rotate" => {
                let (dir, rest) = args.split_once(" ").unwrap();
                let (idx, amount) = rest.split_once(" by ").unwrap();
                let idx = idx.split_once("=").unwrap().1.parse::<usize>().unwrap();
                let amount = amount.parse::<usize>().unwrap();

                display_tmp = display;
                match dir {
                    "row" => {
                        for i in 0..display[0].len() {
                            let next_idx = (i + amount) % display[0].len();
                            display_tmp[idx][next_idx] = display[idx][i];
                        }
                    },
                    "column" => {
                        for i in 0..display.len() {
                            let next_idx = (i + amount) % display.len();
                            display_tmp[next_idx][idx] = display[i][idx];
                        }
                    }
                    _ => unreachable!()
                }

                display = display_tmp;
            }
            _ => unreachable!()
        }
        draw_screen(&display);
    }

    display.iter().map(|line| line.iter().filter(|p| **p).count()).sum()
}

pub fn main() {
    let input: Vec<_> = aoc::get_input_file_lines().collect();
    aoc::run(|| (solve(&input), "Read screen"));
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn test() {
        let pt1 = solve("\
rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1
".lines().collect::<Vec<_>>().as_slice());
        assert_eq!(pt1, 6);
    }
}
