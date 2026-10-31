fn count_triangles(dirs: &[(i32, i32, i32)]) -> usize {
    dirs.iter().copied().filter(|&(a, b, c)| {
        a + b > c && b + c > a &&  a + c > b
    }).count()
}

pub fn main() {
    let input: Vec<_> = aoc::get_input_file_lines()
    .map(|l| {
        let mut l = l.split_whitespace();
        let a = l.next().unwrap().parse::<i32>().unwrap();
        let b = l.next().unwrap().parse::<i32>().unwrap();
        let c = l.next().unwrap().parse::<i32>().unwrap();
        (a, b, c)
    }).collect();

    let input_pt2: Vec<_> = input.chunks(3).flat_map(|c| {
        let [l1, l2, l3] = c else { unreachable!() };
        [ (l1.0, l2.0, l3.0),
          (l1.1, l2.1, l3.1),
          (l1.2, l2.2, l3.2) ]
    })
    .collect();

    aoc::run(|| {
        (count_triangles(&input), count_triangles(&input_pt2))
    });
}

#[cfg(test)]
mod test {
    use crate::count_triangles;

    #[test]
    fn test() {
        assert_eq!(count_triangles(&[(5, 10, 25)]), 0);
    }
}
