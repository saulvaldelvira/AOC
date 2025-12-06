
use core::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use aoc::get_input_file_lines;

struct TSP {
    nodes: HashMap<usize, String>,
    paths: HashMap<usize, Vec<(usize, usize)>>,
}

#[derive(Clone)]
struct TSPEval<'a> {
    graph: &'a TSP,
    visited: HashSet<usize>,
    #[cfg(debug_assertions)]
    path: Vec<usize>,
    cost: usize,
    cmp: fn (usize, usize) -> Ordering,
}

impl TSPEval<'_> {
    fn visit(mut self, node: usize) -> Self {
        self.visited.insert(node);
        #[cfg(debug_assertions)]
        self.path.push(node);

        let Some(paths) = self.graph.paths.get(&node) else { return self };

        paths.iter()
            .filter(|(dst, _)| !self.visited.contains(dst))
            .map(|(dst, cost)| {
                let mut tsp_eval = self.clone();
                tsp_eval.cost += cost;
                tsp_eval.visit(*dst)
            })
            .min_by(|a, b| (self.cmp)(a.cost, b.cost))
            .unwrap_or(self)
    }
}

impl TSP {
    fn visit_all(&self, start: usize, cmp: fn (usize, usize) -> Ordering) -> Option<TSPEval<'_>> {
        let mut eval = TSPEval {
            graph: self,
            visited: HashSet::new(),
            #[cfg(debug_assertions)]
            path: Vec::new(),
            cost: 0,
            cmp
        };
        eval = eval.visit(start);
        for id in self.nodes.keys() {
            if !eval.visited.contains(id) {
                return None
            }
        }
        Some(eval)
    }
}

fn build_graph<I, S>(lines: I) -> TSP
where
    I: Iterator<Item = S>,
    S: AsRef<str>
{
    let mut names = HashMap::new();
    let mut nodes = HashMap::new();
    let mut paths: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut ids = 0_usize;

    let mut id_for = |s: &str| {
        match names.get(s) {
            Some(id) => *id,
            None => {
                ids += 1;
                names.insert(s.to_string(), ids);
                nodes.insert(ids, s.to_string());
                ids
            }
        }
    };

    for line in lines {
        let line = line.as_ref();
        let (path, val) = line.split_once(" = ").unwrap();
        let (src, dst) = path.split_once(" to ").unwrap();
        let val = val.parse::<usize>().unwrap();

        let src = id_for(src);
        let dst = id_for(dst);

        paths.entry(src).or_default().push((dst, val));
        paths.entry(dst).or_default().push((src, val));
    }

    TSP { nodes, paths }
}

fn solve<I, S>(lines: I) -> (usize, usize)
where
    I: Iterator<Item = S>,
    S: AsRef<str>
{
    let tsp = build_graph(lines);

    let eval = tsp.nodes.keys()
        .filter_map(|id| tsp.visit_all(*id, |a, b| a.cmp(&b)))
        .min_by_key(|tsp| tsp.cost)
        .unwrap();
    let part1 = eval.cost;
    visualize("Shortest", &eval);

    let eval = tsp.nodes.keys()
        .filter_map(|id| tsp.visit_all(*id, |a, b| b.cmp(&a)))
        .max_by_key(|tsp| tsp.cost)
        .unwrap();
    let part2 = eval.cost;
    visualize("Longest", &eval);

    (part1, part2)
}

fn main() {
    let input = get_input_file_lines();
    aoc::run(|| solve(input));
}

#[cfg(not(debug_assertions))]
fn visualize(_: &str, _: &TSPEval<'_>) { }

#[cfg(debug_assertions)]
fn visualize(name: &str, tsp: &TSPEval<'_>) {
    print!("{name}: ");
    for (i, p) in tsp.path.iter().enumerate() {
        let name = tsp.graph.nodes.get(p).unwrap();
        if i > 0 { print!("-> ") }
        print!("{name} ")
    }
    println!("= {}", tsp.cost);
}


#[cfg(test)]
mod test {

    #[test]
    fn example() {
        let (part1, part2) = super::solve("London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
.lines());

        assert_eq!(part1, 605);
        assert_eq!(part2, 982);
    }
}
