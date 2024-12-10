
fn solve(src: &str) -> (u64,u64) {
    let mut id = 0;

    let mut result: Vec<i32> = Vec::new();

    let mut is_file = true;
    for c in src.chars() {
        if is_file {
            for _ in 0..(c as u8 - b'0') {
                result.push(id);
            }
            id += 1;
        } else {
            for _ in 0..(c as u8 - b'0') {
                result.push(-1);
            }
        }
        is_file = !is_file;
    }

    let mut result_p2 = result.clone();

    let mut back = result.len() - 1;
    let mut front = 0;
    loop {
        while result[front] != -1 { front += 1; }
        while result[back] == -1 { back -= 1; }
        if front > back { break }
        while result[front] == -1 && result[back] != -1 {
            result[front] = result[back];
            result[back] = -1;
            front += 1;
            back -= 1;
        }
    }

    let mut back = (result_p2.len(), result_p2.len());
    'main: loop {
        back.1 = back.0 - 1;
        while result_p2[back.1] == -1 {
            back.1 -= 1;
        }
        let curr = result_p2[back.1];
        back.0 = back.1;
        while back.0 > 0 && result_p2[back.0] == result_p2[back.1] {
            back.0 -= 1;
        }
        back.0 += 1;

        let mut front = (0,0);
        loop {
            while front.0 < result_p2.len() && result_p2[front.0] != -1 {
                front.0 += 1;
            }
            front.1 = front.0;

            while front.1 < result_p2.len() && result_p2[front.1] == -1 {
                front.1 += 1;
            }
            front.1 -= 1;
            if front.0 > front.1 || back.0 > back.1 { continue 'main }
            if (front.1 - front.0) >= (back.1 - back.0) { break }
            front.0 = front.1 + 1;
        }

        if front.1 > back.0 {
            if curr == 0 { break }
            else { continue }
        }

        for (f,b) in (front.0..=front.1).zip((back.0)..=back.1) {
            result_p2[f] = result_p2[b];
            result_p2[b] = -1;
        }

        front.0 += back.1 - back.0;
    }

    let sumup = |res: &Vec<i32>| {
        res.iter().enumerate().filter(|(_,n)| **n != -1).fold(0, |acc, (i,n)| {
            acc + (i as i32 * n) as u64
        })
    };

    (
        sumup(&result),
        sumup(&result_p2),
    )
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let (part1,part2) = solve("2333133121414131402");
        assert_eq!(part1,1928);
        assert_eq!(part2,2858);
    }
}
