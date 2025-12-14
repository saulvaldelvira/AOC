use itertools::Itertools;

fn solve(input: &[String]) -> (usize, usize) {

    struct Ingredient {
        capacity: i64,
        durability: i64,
        flavor: i64,
        texture: i64,
        calories: i64,
    }

    let ingredients: Vec<_> =
        input.iter()
        .map(|line| {
            let (_, rest) = line.split_once(": ").unwrap();

            let mut values = rest.trim().split(", ");
            let mut parse_next = || -> i64 {
                values.next().unwrap()
                    .split_once(" ").unwrap().1
                    .parse::<i64>().unwrap()
            };

           Ingredient {
               capacity: parse_next(),
               durability: parse_next(),
               flavor: parse_next(),
               texture: parse_next(),
               calories: parse_next(),
           }

        })
        .collect();

    let max = (0..=100).permutations(ingredients.len())
        .filter(|amounts| {
            amounts.iter().copied().sum::<i64>() == 100
        })
        .map(|amounts| {
            let (a, b, c, d, kcal) =
                ingredients.iter().zip(amounts.iter()).map(|(ing, am)| {
                let cap = ing.capacity * am;
                let dur = ing.durability * am;
                let flavour = ing.flavor * am;
                let texture = ing.texture * am;
                let cal = ing.calories * am;
                (cap, dur, flavour, texture, cal)
            })
            .fold((0, 0, 0, 0, 0), |a, b| {
                (
                    a.0 + b.0,
                    a.1 + b.1,
                    a.2 + b.2,
                    a.3 + b.3,
                    a.4 + b.4,
                )
            });
            let total = a.max(0) * b.max(0) * c.max(0) * d.max(0);
            (total, kcal)
        });

    let part1 = max.clone().max_by_key(|(total, _)| *total).unwrap().0;
    let part2 = max.filter(|&(_, cal)| cal == 500).max_by_key(|(total, _)| *total).unwrap().0;

    (part1 as _, part2 as _)
}

fn main() {
    let input: Vec<String> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let (part1, part2) = super::solve("\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
.lines()
.map(str::to_string)
.collect::<Vec<_>>()
.as_slice());

        assert_eq!(part1, 62842880);
        assert_eq!(part2, 57600000);
    }
}
