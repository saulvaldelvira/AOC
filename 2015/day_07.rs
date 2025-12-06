use core::hash::Hash;
use std::collections::HashMap;

use aoc::get_input_file_lines;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum Input {
    Value(u16),
    Gate(String),
    Not(Box<Input>),
    And(Box<Input>, Box<Input>),
    Or(Box<Input>, Box<Input>),
    LShift(Box<Input>, u16),
    RShift(Box<Input>, u16),
}

impl Input {
    fn eval(&self, values: &mut HashMap<String, u16>, gates: &HashMap<String, Input>) -> u16 {
        match self {
            Input::Value(num) => *num,
            Input::Gate(g) =>{
                if let Some(val) = values.get(g) {
                    *val
                } else {
                    let val = gates.get(g).unwrap().eval(values, gates);
                    values.insert(g.to_string(), val);
                    val
                }
            }
            Input::And(l, r) => l.eval(values, gates) & r.eval(values, gates),
            Input::Or(l, r) => l.eval(values, gates) | r.eval(values, gates),
            Input::LShift(gate, num) => gate.eval(values, gates) << num,
            Input::RShift(gate, num) => gate.eval(values, gates) >> num,
            Input::Not(gate) => !gate.eval(values, gates),
        }
    }

    fn new(s: &str) -> Self {
        if let Some((left, right)) = s.split_once(" AND ") {
            Input::And(Self::new(left).into(), Self::new(right).into())
        } else if let Some((left, right)) = s.split_once(" OR ") {
            Input::Or(Self::new(left).into(), Self::new(right).into())
        } else if let Some((left, right)) = s.split_once(" LSHIFT ") {
            Input::LShift(Self::new(left).into(), right.parse().unwrap())
        } else if let Some((left, right)) = s.split_once(" RSHIFT ") {
            Input::RShift(Self::new(left).into(), right.parse().unwrap())
        } else if let Some(s) = s.strip_prefix("NOT ") {
            Input::Not(Self::new(s).into())
        } else if let Ok(n) = s.parse() {
            Input::Value(n)
        } else {
            Input::Gate(s.to_string())
        }
    }
}

fn get_gates<I, S>(lines: I) -> HashMap<String, Input>
where
    I: Iterator<Item = S>,
    S: AsRef<str>
{
    let mut wires = HashMap::new();

    for line in lines {
        let (input, target) = line.as_ref().trim().split_once(" -> ").unwrap();
        wires.insert(target.to_string(), Input::new(input));
    }

    wires
}

fn solve<I, S>(lines: I) -> (usize, usize)
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let gates = get_gates(lines);

    let mut values = HashMap::new();
    let part1 = gates.get("a").unwrap().eval(&mut values, &gates);

    values.clear();
    values.insert("b".to_string(), part1);
    let part2 = gates.get("a").unwrap().eval(&mut values, &gates);

    (part1 as usize, part2 as usize)
}

fn main() {
    let input = get_input_file_lines();
    aoc::run(|| solve(input));
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn example() {
        let gates = super::get_gates("123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i".lines());

        const EXPECTED: &[(&str, u16)] = &[
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];

        let mut values = HashMap::new();
        for (s, n) in EXPECTED {
            let eval = gates.get(*s).unwrap().eval(&mut values, &gates);
            assert_eq!(*n, eval);
        }
    }
}
