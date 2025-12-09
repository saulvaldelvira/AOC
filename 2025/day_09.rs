use core::ops::RangeInclusive;
use aoc::{pairs_wrap, permutations};

#[derive(PartialEq, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    /* pnpoly: https://wrfranklin.org/Research/Short_Notes/pnpoly.html */
    fn is_inside_polygon(&self, p: &Polygon) -> bool {
        let mut inside = false;

        for (a, b) in pairs_wrap(&p.points) {
            if (a.y > self.y) != (b.y > self.y)
                && self.x < (b.x - a.x) * ( self.y - a.y) / (b.y - a.y) + a.x
            {
                inside = !inside;
            }
        }

        inside
    }
}

struct Polygon {
    points: Vec<Vec2>,
}

#[derive(Clone, Copy)]
struct Rectangle {
    tl: Vec2,
    br: Vec2,
}

struct Line {
    start: Vec2,
    end: Vec2,
}

impl Line {
    const fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }
    fn over_x(&self) -> Option<i64> { (self.start.y == self.end.y).then_some(self.end.y) }
    fn over_y(&self) -> Option<i64> { (self.start.x == self.end.x).then_some(self.end.x) }
    fn range_y(&self) -> RangeInclusive<i64> {
        let start = self.start.y.min(self.end.y);
        let end = self.start.y.max(self.end.y);
        start..=end
    }
    fn range_x(&self) -> RangeInclusive<i64> {
        let start = self.start.x.min(self.end.x);
        let end = self.start.x.max(self.end.x);
        start..=end
    }
    fn intersects_once(&self, other: &Line) -> bool {
        if let Some(selfy) = self.over_x() {
            let Some(otherx) = other.over_y() else { return false };
            other.range_y().contains(&selfy)
            && self.range_x().contains(&otherx)
        } else if let Some(selfx) = self.over_y() {
            let Some(othery) = other.over_x() else { return false };
            self.range_y().contains(&othery)
            && other.range_x().contains(&selfx)
        } else {
            unreachable!("Lines cannot be diagonal")
        }
    }
}

impl Rectangle {
    const fn width(&self) -> i64 { (self.br.x - self.tl.x).abs() + 1 }
    const fn height(&self) -> i64 { (self.tl.y - self.br.y).abs()  + 1 }
    const fn area(&self) -> i64 {
        self.width() * self.height()
    }

    const fn points(&self) -> [Vec2; 4] {
        let mut tr = self.tl;
        tr.x = self.br.x;
        let mut bl = self.tl;
        bl.y = self.br.y;
        [ tr, bl, self.tl, self.br ]
    }

    const fn lines(&self) -> [Line; 4] {
        let [tr, bl, tl, br ] = self.points();

        [ Line::new(tl, tr),
          Line::new(tl, bl),
          Line::new(bl, br),
          Line::new(tr, br),
        ]
    }

    fn shrink(&self) -> Self {
        let mut c = *self;
        c.tl.x += 1;
        c.tl.y += 1;
        c.br.x -= 1;
        c.br.y -= 1;
        c
    }

    fn inside_polygon(&self, p: &Polygon) -> bool {
        for po in self.points() {
            if !po.is_inside_polygon(p) { return false }
        }

        for (a, b) in pairs_wrap(&p.points) {
            let line = Line::new(*a, *b);

            for sline in self.lines() {
                if line.intersects_once(&sline) { return false }
            }
        }

        true
    }
}

fn solve(input: &[impl AsRef<str>]) -> (usize, usize) {
    let tiles: Vec<_> = input.iter().map(|l| {
        let (col, row) = l.as_ref().split_once(",").unwrap();
        let x = col.parse().unwrap();
        let y = row.parse().unwrap();
        Vec2 { x, y }
    })
    .collect();

    let poly = Polygon { points: tiles };

    let mut recs: Vec<_> = permutations(&poly.points)
        .map(|(a, b)| {
            let minx = a.x.min(b.x);
            let miny = a.y.min(b.y);
            let maxx = a.x.max(b.x);
            let maxy = a.y.max(b.y);

            let tl = Vec2 { x: minx, y: miny };
            let br = Vec2 { x: maxx, y: maxy };

            Rectangle { tl, br }
        })
        .collect();

    recs.sort_by_key(|b| std::cmp::Reverse(b.area()));

    let part1 = recs.first().unwrap();

    let part2 = recs.iter()
    .find(|rect| rect.shrink().inside_polygon(&poly))
    .unwrap();

    (part1.area() as usize, part2.area() as usize)
}

fn main() {
    let input: Vec<_> = aoc::get_input_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let (part1, part2) = solve(&"\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
.lines()
.collect::<Vec<_>>());
        assert_eq!(part1, 50);
        assert_eq!(part2, 24);
    }

    #[test]
    fn example2() {
        let (_, part2) = solve(&"\
1,0
3,0
3,6
16,6
16,0
18,0
18,9
13,9
13,7
6,7
6,9
1,9"
.lines()
.collect::<Vec<_>>());
        assert_eq!(part2, 30);
    }
}
