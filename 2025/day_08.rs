use std::collections::HashMap;
use std::time::Instant;
use aoc::permutations;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    const fn euclidean_distance(&self, other: &Self) -> i64 {
        ( (other.x - self.x).pow(2)
          + (other.y - self.y).pow(2)
          + (other.z - self.z).pow(2)
        )
        .isqrt()
    }
}

#[derive(Debug)]
struct Circuit<'a> {
    elements: Vec<&'a Vec3>,
    id: usize,
}

struct Context<'a> {
    circuit_of: HashMap<&'a Vec3, usize>,
    circuits: HashMap<usize, Circuit<'a>>,
    combinations: Vec<(&'a Vec3, &'a Vec3)>,
}

fn build_jboxes(lines: &[impl AsRef<str>]) -> Vec<Vec3> {
    lines.iter().map(|line| {
        let mut line = line.as_ref().split(",");
        let x = line.next().unwrap().parse::<i64>().unwrap();
        let y = line.next().unwrap().parse::<i64>().unwrap();
        let z = line.next().unwrap().parse::<i64>().unwrap();
        Vec3 { x, y, z }
    })
    .collect()
}

fn build_context(jboxes: &[Vec3]) -> Context<'_> {
    let mut circuits = HashMap::new();
    let mut circuit_of = HashMap::<&Vec3, usize>::new();
    for (id, jbox) in jboxes.iter().enumerate() {
        circuits.insert(id, Circuit { id, elements: vec![jbox] });
        circuit_of.insert(jbox, id);
    }

    let mut combinations: Vec<_> = permutations(jboxes).collect();
    combinations.sort_by_key(|(a, b)| a.euclidean_distance(b));

    Context {
        circuit_of,
        combinations,
        circuits
    }

}

fn solve(n_iter_pt1: usize, context: Context<'_>) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    let Context { mut circuits, combinations, mut circuit_of } = context;

    for (i, (a, b)) in combinations.iter().enumerate() {
        if i == n_iter_pt1 {
            let mut circuits: Vec<_> = circuits.values().collect();
            circuits.sort_by(|a, b| b.elements.len().cmp(&a.elements.len()));

            part1 = circuits[..3].iter()
                .map(|c| c.elements.len())
                .reduce(|a, b| a * b)
                .unwrap();
        }

        let a_circuit_id = circuit_of.get(a).unwrap();
        let b_circuit_id = circuit_of.get(b).unwrap();
        if a_circuit_id == b_circuit_id { continue }

        let b_circuit = circuits.remove(b_circuit_id).unwrap();
        let a_circuit = circuits.get_mut(a_circuit_id).unwrap();

        for elem in b_circuit.elements {
            a_circuit.elements.push(elem);
            circuit_of.insert(elem, a_circuit.id);
        }

        if circuits.len() == 1 {
            part2 = (a.x * b.x) as usize;
        }
    }

    (part1, part2)
}


fn main() {
    let start = Instant::now();
    let input: Vec<_> = aoc::get_input_lines().collect();
    let jboxes = build_jboxes(&input);
    let ctx = build_context(&jboxes);
    let end = start.elapsed();

    print!("Parsing time: ");
    aoc::print_time(end);
    println!();

    aoc::run(|| solve(1000, ctx));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let jboxes = build_jboxes(&"\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
.lines()
.collect::<Vec<_>>());
        let ctx = build_context(&jboxes);
        let (part1, part2) = solve(10, ctx);
        assert_eq!(part1, 40);
        assert_eq!(part2, 25272);
    }
}
