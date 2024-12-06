
fn transmute(lines: &[String]) -> Vec<String> {
    let mut res = Vec::new();
    for i in 0..lines.len() {
        let mut s = String::new();
        for j in 0..lines.len() {
            s.push_str(&lines[j][i..=i]);
        }
        res.push(s);
    }
    res
}

fn for_each_chunk<const STEP: usize>(lines: &[String], mut f: impl FnMut(&[char],&[char])) {
    for x in 0..lines.len() - (STEP - 1) {
        for y in 0..lines.len() - (STEP - 1) {
            let mut diag1 = ['0'; STEP];
            let mut diag2 = ['0'; STEP];

            for i in 0..STEP {
                diag1[i] = lines[y + i][x + i..=x+i].chars().next().unwrap();
                let y = y + i;
                let x = x + STEP - 1 - i;
                diag2[i] = lines[y][x..=x].chars().next().unwrap();
            }

            f(&diag1,&diag2);
        }
    }
}

pub fn main() {
    let lines: Vec<String> =
        aoc::get_input_file_lines().collect();

    let mut count = 0;

    for l in &lines {
        count +=
            l.matches("XMAS").count()
            + l.matches("SAMX").count()
    }

    let cols = transmute(&lines);

    for c in &cols {
        count +=
            c.matches("XMAS").count()
            + c.matches("SAMX").count()
    }


    for_each_chunk::<4>(&lines, |d1,d2| {
        for d in [d1,d2] {
            if d == ['X','M','A','S']
               || d == ['S', 'A', 'M', 'X'] {
                   count += 1;
            }
        }
    });

    println!("Part 1: {count}");

    count = 0;
    for_each_chunk::<3>(&lines, |d1,d2| {
        const MAS: [char; 3] = ['M','A','S'];
        const SAM: [char; 3] = ['S','A','M'];
        if (d1 == MAS || d1 == SAM) && (d2 == MAS || d2 == SAM) {
            count += 1;
        }
    });

    println!("Part 2: {count}");
}
