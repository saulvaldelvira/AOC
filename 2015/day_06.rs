
fn str_to_coords(src: &str) -> (usize,usize) {
    let mut src = src.split(",");
    let x = src.next().unwrap().parse::<usize>().unwrap();
    let y = src.next().unwrap().parse::<usize>().unwrap();
    (x,y)
}

fn solve(src: &str) -> (usize,i32) {
    let mut lights_p1 = [[false; 1000]; 1000];
    let mut lights_p2 = [[0_i32; 1000]; 1000];

    #[derive(Clone,Copy)]
    enum Order {
        On, Off, Toggle
    }

    let prefixes = [
        ("turn on ", Order::On),
        ("turn off ", Order::Off),
        ("toggle ", Order::Toggle),
    ];

    for mut line in src.lines().filter(|l| !l.is_empty()) {
        let mut order = Order::On;
        for (p, v) in &prefixes {
            if line.starts_with(p) {
                line = line.strip_prefix(p).unwrap();
                order = *v;
            }
        };

        let mut coords = line.split(" through ");
        let from = coords.next().unwrap();
        let to = coords.next().unwrap();

        let from = str_to_coords(from);
        let to = str_to_coords(to);
        let mut curr = from;

        loop {
            if curr.0 > to.0 {
                curr.0 = from.0;
                curr.1 += 1;
                if curr.1 > to.1 { break }
            }

            let val = &mut lights_p1[curr.0][curr.1];
            match order {
                Order::On => *val = true,
                Order::Off => *val = false,
                Order::Toggle => *val = !*val,
            }
            let val = &mut lights_p2[curr.0][curr.1];
            match order {
                Order::On => *val += 1,
                Order::Off => if *val > 0 { *val -= 1 },
                Order::Toggle => *val += 2,
            }

            curr.0 += 1;
        }
    }

    let part1 = lights_p1.iter().map(|line| {
        line.iter().filter(|&&on| on).count()
    }).sum();

    let part2 = lights_p2.iter().map(|line| {
        line.iter().sum::<i32>()
    }).sum();

    (part1,part2)
}

pub fn main() {
    let (part1,part2) = solve(include_str!("input.txt"));
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
