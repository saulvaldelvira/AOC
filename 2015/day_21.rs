use itertools::Itertools;
use aoc::{dprint, dprintln};

fn simulate(
    mut my_points: u16,
    mut enemy_points: u16,
    my_stats: &Stats,
    enemy: &Stats
) -> bool
{
    dprintln!("Match Me ({my_points}, {my_stats:?}) VS Enemy ({enemy_points}, {enemy:?})");
    let i_deal = my_stats.damage.checked_sub(enemy.armor).unwrap_or(1).max(1);
    let enemy_deals = enemy.damage.checked_sub(my_stats.armor).unwrap_or(1).max(1);
    loop {
        dprint!("	I deal {i_deal} ");
        if enemy_points <= i_deal {
            dprintln!("	I WIN");
            return true
        }
        dprintln!(". Enemy {enemy_points} => {}", enemy_points - i_deal);
        enemy_points -= i_deal;

        dprint!("	He deals {enemy_deals}. ");
        if my_points <= enemy_deals {
            dprintln!("	I LOOSE");
            return false
        }
        dprintln!("	I {my_points} => {}", my_points - enemy_deals);
        my_points -= enemy_deals;
    }
}

#[derive(Clone, Copy, Debug)]
struct Stats {
    cost: u16,
    damage: u16,
    armor: u16,
}

fn solve(lines: &[String]) -> (usize, usize) {
    let weapons : &[Stats] = &[
        (8, 4, 0),
        (10,  5, 0),
         (25, 6, 0),
         (40, 7, 0),
         (74, 8, 0)
    ].map(|(cost, damage, armor)| Stats { cost, damage, armor });

    let armors: &[Stats] = &[
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ].map(|(cost, damage, armor)| Stats { cost, damage, armor });

    let rings: &[Stats] = &[
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ].map(|(cost, damage, armor)| Stats { cost, damage, armor });

    let rings_combinations = rings.iter().powerset().filter(|s| s.len() <= 2);

    let mut all_combinations: Vec<_> = rings_combinations
        .cartesian_product(armors.iter().map(Some).chain([None]))
        .cartesian_product(weapons.iter().map(Some))
        .map(|((rings, armor), weapon)| {
            let items = rings.iter().copied().map(Some).chain([armor, weapon])
                .flatten();
            Stats {
                armor: items.clone().map(|i| i.armor).sum(),
                cost: items.clone().map(|i| i.cost).sum(),
                damage: items.clone().map(|i| i.damage).sum(),
            }
        })
        .collect();

    all_combinations.sort_by_key(|s| s.cost);

    let mut enemy_stats = Stats { cost: 0, damage: 0, armor: 0 };
    let mut enemy_points = 0;
    for line in lines {
        if let Some(hit) = line.strip_prefix("Hit Points: ") {
            enemy_points = hit.parse().unwrap();
        }
        else if let Some(dam) = line.strip_prefix("Damage: ") {
            enemy_stats.damage = dam.parse().unwrap();
        }
        else if let Some(armor) = line.strip_prefix("Armor: ") {
            enemy_stats.armor = armor.parse().unwrap();
        }
    }

    let part1 = all_combinations.iter()
        .find(|stats| simulate(100, enemy_points, stats, &enemy_stats))
        .unwrap()
        .cost as usize;

    let part2 = all_combinations.iter().rev()
        .find(|stats| !simulate(100, enemy_points, stats, &enemy_stats))
        .unwrap()
        .cost as usize;

    (part1, part2)
}

fn main() {
    let input: Vec<String> = aoc::get_input_file_lines().collect();
    aoc::run(|| solve(&input));
}

#[cfg(test)]
mod test {
    use crate::Stats;


    #[test]
    fn example() {
        let my_stats = Stats {
            damage: 5,
            armor: 5,
            cost: 0
        };
        let enemy_stats = Stats {
            damage: 7,
            armor: 2,
            cost: 0
        };

        let win = super::simulate(8, 12, &my_stats, &enemy_stats);
        assert!(win);

    }
}
