
#[derive(Clone, Copy)]
struct State {
    my_points: i16,
    enemy_points: i16,
    my_mana: i16,
    spent_mana: u32,
    enemy_damage: i16,
    shield: i16,
    poison: i16,
    recharge: i16,
}

fn explore(mut state: State, is_pt2: bool, best: &mut Option<u32>, my_turn: bool) {
    // Checks
    if state.my_points <= 0 {
        return;
    }
    if let Some(b) = best && state.spent_mana >= *b {
        return;
    }
    if my_turn && is_pt2 {
        state.my_points -= 1;
    }

    // Effects
    let shield = state.shield > 0;
    if shield {
        state.shield -= 1;
    }
    if state.poison > 0 {
        state.poison -= 1;
        state.enemy_points -= 3;
    }
    if state.recharge  > 0 {
        state.my_mana += 101;
        state.recharge -= 1;
    }

    if state.enemy_points <= 0 {
        if let Some(b) = best && *b > state.spent_mana {
            *b = state.spent_mana;
        } else {
            *best = Some(state.spent_mana);
        }
        return;
    }

    // Cast
    if my_turn {
        if state.my_mana >= 53 {
            let mut s = state;
            s.my_mana -= 53;
            s.spent_mana += 53;
            s.enemy_points -= 4;
            explore(s, is_pt2, best, false);
        }
        if state.my_mana >= 73 {
            let mut s = state;
            s.my_mana -= 73;
            s.spent_mana += 73;
            s.enemy_points -= 2;
            s.my_points += 2;
            explore(s, is_pt2, best, false);
        }
        if state.my_mana >= 113 && state.shield == 0 {
            let mut s = state;
            s.my_mana -= 113;
            s.spent_mana += 113;
            s.shield = 6;
            explore(s, is_pt2, best, false);
        }
        if state.my_mana >= 173 && state.poison == 0 {
            let mut s = state;
            s.my_mana -= 173;
            s.spent_mana += 173;
            s.poison = 6;
            explore(s, is_pt2, best, false);
        }
        if state.my_mana >= 229 && state.recharge == 0 {
            let mut s = state;
            s.my_mana -= 229;
            s.spent_mana += 229;
            s.recharge = 5;
            explore(s, is_pt2, best, false);
        }
    } else {
        let mut damage = state.enemy_damage;
        if shield { damage -= 7; }
        state.my_points -= damage.max(1);
        explore(state, is_pt2, best, true);
    }
}

fn solve(my_hit_points: i16, my_mana: i16, enemy_hit_points: i16, enemy_damage: i16) -> (u32, u32) {
    let initial = State {
        my_points: my_hit_points,
        enemy_points: enemy_hit_points,
        my_mana,
        spent_mana: 0,
        enemy_damage,
        shield: 0,
        poison: 0,
        recharge: 0,
    };

    let mut best = None;
    explore(initial, false, &mut best, true);
    let pt1 = best.unwrap();
    best = None;
    explore(initial, true, &mut best, true);
    let pt2 = best.take().unwrap_or(0);
    (pt1, pt2)
}

fn main() {
    let mut input = aoc::get_input_file_lines();
    let hit_points: i16 = input.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
    let damage: i16 = input.next().unwrap().split_once(": ").unwrap().1.parse().unwrap();
    aoc::run(|| solve(50, 500, hit_points, damage));
}

#[cfg(test)]
mod test {
    use super::solve;

    #[test]
    fn example() {
        let (part1, _) = solve(10, 250, 13, 8);
        assert_eq!(part1, 226);

        let (part1, _) = solve(10, 250, 14, 8);
        assert_eq!(part1, 641);
    }
}
