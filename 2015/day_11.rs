
fn is_ok(input: &mut [u8]) -> bool {
    if let Some((first, _)) = input.iter()
                                   .enumerate()
                                   .find(|(_, &c)| matches!(c, b'i' | b'l' | b'o'))
    {
        input[first] += 1;
        input.get_mut(first+1..)
             .unwrap_or(&mut [])
             .iter_mut()
             .for_each(|n| *n = b'a');
    }

    let a = input.iter();
    let b = input.iter().skip(1);
    let c = input.iter().skip(2);
    let mut three_row = false;
    for (a, (b, c)) in a.clone().zip(b.clone().zip(c)) {
        if *c == *b + 1 && *b == *a + 1 {
            three_row = true;
            break
        }
    }

    let mut two_same = 0;
    let mut last_match_letter = 0;
    let mut last_match_index = 0;
    for ((i, a), b) in a.enumerate().zip(b) {
        if *a == *b {
            /* Can't overlap */
            if last_match_letter == *a
               && last_match_index == i
            {
                continue;
            }
            two_same += 1;
            last_match_index = i + 1;
            last_match_letter = *a;
        }
    }

    three_row && two_same >= 2
}

fn next_password(input: &mut [u8]) -> String {
    let mut first = true;
    while first || !is_ok(input) {
        first = false;
        let mut it = input.iter_mut().rev();
        let mut carry = true;
        while carry {
            let c = it.next().unwrap();
            if *c == b'z' {
                *c = b'a';
            } else {
                *c += 1;
                carry = false;
            }
        }
    }

    String::from_utf8(input.to_vec()).unwrap()
}

fn solve(input: &mut [u8]) -> (String, String) {
    let part1 = next_password(input);
    let part2 = next_password(input);
    (part1, part2)
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    aoc::run(|| solve(&mut input.into_bytes()));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let next = super::next_password(&mut "abcdefgh".as_bytes().to_vec());
        assert_eq!(next, "abcdffaa");

        let next = super::next_password(&mut "ghijklmn".as_bytes().to_vec());
        assert_eq!(next, "ghjaabcc");
    }
}
