use core::fmt::Display;

#[derive(Debug)]
enum Expansion {
    Repeat(Box<Expansion>, usize),
    Chars(String),
    List(Vec<Expansion>),
}

impl Expansion {
    fn len(&self) -> usize {
        match self {
            Expansion::Repeat(e, n) => e.len() * n,
            Expansion::Chars(c) => c.len(),
            Expansion::List(list) => list.iter().map(Expansion::len).sum::<usize>(),
        }
    }
}

impl Display for Expansion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expansion::Repeat(e, n) => {
                for _ in 0..*n {
                    e.fmt(f)?;
                }
            }
            Expansion::Chars(c) => write!(f, "{c}")?,
            Expansion::List(exp) => {
                for e in exp {
                    e.fmt(f)?;
                }
            }
        }
        Ok(())
    }
}

fn get_segment_dimensions(text: &str, i: &mut usize) -> (usize, usize) {
    let mut segment_size: usize = 0;
    let num_repeats: usize;
    let mut acc = String::new();
    loop {
        let c = text[*i..].chars().next().unwrap();
        *i += 1;
        if c == 'x' {
            segment_size = acc.parse().unwrap();
            acc.clear();
        } else if c == ')' {
            num_repeats = acc.parse().unwrap();
            return (segment_size, num_repeats)
        } else {
            if !c.is_ascii_whitespace() {
                acc.push(c);
            }
        }
    }
}

fn do_parse(text: &str, v1: bool, inside_nest: bool, n: impl Into<Option<usize>>) -> Expansion {
    let mut res = Vec::new();
    let n = n.into().unwrap_or(text.len());

    let mut i = 0;
    let mut str_acc = String::new();
    while i < n {
        let c = text[i..].chars().next().unwrap();
        i += 1;
        if c == '(' {
            if v1 && inside_nest {
                str_acc.push(c);
            } else {
                let (size, num_rep) = get_segment_dimensions(text, &mut i);
                if !str_acc.is_empty() {
                    res.push(Expansion::Chars(str_acc));
                    str_acc = String::new();
                }
                let p = do_parse(&text[i..], v1, true, size);
                i += size;
                res.push(Expansion::Repeat(Box::new(p), num_rep));
            }
        } else {
            if !c.is_ascii_whitespace() {
                str_acc.push(c);
            }
        }
        if i == n { break }
    }

    if !str_acc.is_empty() {
        if res.is_empty() {
            return Expansion::Chars(str_acc)
        } else {
            res.push(Expansion::Chars(str_acc));
        }
    }
    Expansion::List(res)
}

fn parse_v1(text: &str) -> Expansion {
    do_parse(text, true, false, None)
}

fn parse_v2(text: &str) -> Expansion {
    do_parse(text, false, false, None)
}

pub fn main() {
    let input = aoc::get_input_string();
    aoc::run(|| {
        let pt1 = parse_v1(&input).len();
        let pt2 = parse_v2(&input).len();
        (pt1, pt2)
    });
}

#[cfg(test)]
mod test {
    use crate::{parse_v1, parse_v2};

    #[test]
    fn test() {
        const INPUT: &[(&str, &str)] = &[
            ("ADVENT", "ADVENT"),
            ("A(1x5)BC", "ABBBBBC"),
            ("(3x3)XYZ", "XYZXYZXYZ"),
            ("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG"),
            ("(6x1)(1x3)A", "(1x3)A"),
            ("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY"),
        ];

        for (src, expected) in INPUT {
            let dec = parse_v1(src);
            assert_eq!(&dec.to_string(), expected);
        }

        let res = parse_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A");
        assert_eq!(res.len(), 241920);
        let res = parse_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");
        assert_eq!(res.len(), 445);

    }
}
