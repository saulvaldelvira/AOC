use json::Json;

fn solve(input: &str) -> (usize, usize) {
    let json = Json::deserialize(input).unwrap();

    const fn sum_tuple(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
        (a.0 + b.0, a.1 + b.1)
    }

    fn get_sum(json: &Json) -> (f64, f64) {
        match json {
            Json::Array(jsons) => jsons.iter()
                                       .map(get_sum)
                                       .fold((0.0, 0.0), sum_tuple),
            Json::Object(obj) => {
                let mut has_red = false;
                let mut sum = obj.values().map(|val| {
                    if val.string().is_some_and(|s| s == "red") {
                        has_red = true;
                    }
                    get_sum(val)
                })
                .fold((0.0, 0.0), sum_tuple);
                if has_red {
                    sum.1 = 0.0;
                }
                sum
            }
            Json::Number(n) => (*n, *n),
            Json::True |
            Json::False |
            Json::Null |
            Json::String(_) => (0.0, 0.0),
        }
    }

    let (part1, part2) = get_sum(&json);
    (part1 as usize, part2 as usize)
}

fn main() {
    let input = aoc::get_input_string();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        const EXPECTED: &[(&str, usize, usize)] = &[
            ("[1,2,3]", 6, 6),
            (r#"{"a":2,"b":4}"#, 6, 6),
            (r#"[1,{"c":"red","b":2},3]"#, 6, 4),
        ];

        for (a, exp1, exp2) in EXPECTED {
            let (part1, part2) = super::solve(a);
            assert_eq!(part1, *exp1);
            assert_eq!(part2, *exp2);
        }
    }
}
