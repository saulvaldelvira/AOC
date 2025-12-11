use std::collections::{HashMap, VecDeque};

type NodesMap = HashMap<u32, Vec<u32>>;

fn solve_pt1(src: u32, dst: u32, nodes: &NodesMap) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(src);
    let mut count = 1;
    while let Some(node) = queue.pop_front() {
        if node == dst{ continue }

        let Some(outs) = nodes.get(&node) else { continue };
        count += outs.len() - 1;

        for &node in outs {
            queue.push_front(node);
        }
    }
    count
}

type ValidFromKey<const N: usize> = (u32, [(u32, bool); N]);

fn solve_pt2<const N: usize>(
    node: u32,
    end: u32,
    must: [(u32, bool); N],
    valid_from: &mut HashMap<ValidFromKey<N>, usize>,
    nodes: &NodesMap)  -> usize
{
    if let Some(c) = valid_from.get(&(node, must)) {
        return *c;
    }

    let Some(outs) = nodes.get(&node) else { return 0 };

    let mut count = 0;
    for &child in outs {
        if child == end {
            if must.iter().all(|(_, is)| *is) {
                count += 1;
            }
        } else {

            let mut new_must = [(0, false); N];
            for (i, (id, is)) in must.iter().enumerate() {
                new_must[i] = (*id, *is || node == *id);
            }
            count += solve_pt2(
                child,
                end,
                new_must,
                valid_from,
                nodes
            );
        }
    }
    *valid_from.entry((node, must)).or_default() = count;
    count
}

fn solve(input: &[impl AsRef<str>]) -> (usize, usize) {
    let mut ids: HashMap<String, u32> = HashMap::new();
    let mut names = HashMap::new();
    let mut nodes: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut id_count = 1;

    let mut get_or_insert = |id: &str| {
        match ids.get(id) {
            Some(id) => *id,
            None => {
                names.insert(id_count, id.to_string());
                ids.insert(id.to_string(), id_count);
                id_count += 1;
                id_count - 1
            }
        }
    };

    for line in input {
        let (src, dsts)= line.as_ref().split_once(": ").unwrap();
        let src_id = get_or_insert(src);
        let outs = dsts.split_whitespace()
            .map(&mut get_or_insert)
            .collect();

        nodes.insert(src_id, outs);
    }

    let you = get_or_insert("you");
    let out_id = get_or_insert("out");

    let part1 = solve_pt1(you, out_id, &nodes);

    let src = get_or_insert("svr");
    let fft_id = get_or_insert("fft");
    let dac_id = get_or_insert("dac");

    let mut valids = HashMap::new();
    let must = [(fft_id, false), (dac_id, false)];
    let part2 = solve_pt2(src, out_id, must, &mut valids, &nodes);

    (part1, part2)
}

fn main() {
    let input: Vec<_> = aoc::get_input_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let (part1, _) = solve(&"\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
.lines()
.collect::<Vec<_>>());
        assert_eq!(part1, 5);
    }

    #[test]
    fn part2() {
        let (_, part2) = solve(&"\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
.lines()
.collect::<Vec<_>>());
        assert_eq!(part2, 2);
    }
}
