use crate::utils;
extern crate itertools;
use itertools::Itertools;
use std::cmp::{max, min};

// these come from the puzzle

const PLAYER_HP: i32 = 100;

// (cost, damage, armor)
const WEAPONS: [(i32, i32, i32); 5] = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (78, 8, 0)];

// (cost, damage, armor)
const ARMORS: [(i32, i32, i32); 5] = [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];

// (cost, damage, armor)
const RINGS: [(i32, i32, i32); 6] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

fn player_wins_fight(php: i32, pd: i32, pa: i32, bhp: i32, bd: i32, ba: i32) -> bool {
    let mut php = php;
    let mut bhp = bhp;

    loop {
        bhp -= max(1, pd - ba);
        if bhp <= 0 {
            return true;
        }

        php -= max(1, bd - pa);
        if php <= 0 {
            return false;
        }
    }
}

fn find_optimal_equipment(boss: (i32, i32, i32)) -> (i32, i32) {
    let mut optimal_cost = std::i32::MAX;
    let mut worst_cost = 0;

    for weapon in WEAPONS.iter() {
        for a in -1..(ARMORS.len() as i32) {
            let armor = if a == -1 {
                (0, 0, 0)
            } else {
                ARMORS[a as usize]
            };

            for num_rings in 0..=2 {
                let ring_combinations: Vec<Vec<(i32, i32, i32)>> = (0..RINGS.len())
                    .combinations(num_rings)
                    .map(|c| c.into_iter().map(|ridx| RINGS[ridx]).collect())
                    .collect();

                for rings in ring_combinations {
                    let cost: i32 = weapon.0 + armor.0 + rings.iter().map(|r| r.0).sum::<i32>();
                    let player_damage = weapon.1 + armor.1 + rings.iter().map(|r| r.1).sum::<i32>();
                    let player_armor = weapon.2 + armor.2 + rings.iter().map(|r| r.2).sum::<i32>();

                    if player_wins_fight(
                        PLAYER_HP,
                        player_damage,
                        player_armor,
                        boss.0,
                        boss.1,
                        boss.2,
                    ) {
                        optimal_cost = min(cost, optimal_cost);
                    }

                    if !player_wins_fight(
                        PLAYER_HP,
                        player_damage,
                        player_armor,
                        boss.0,
                        boss.1,
                        boss.2,
                    ) {
                        worst_cost = max(cost, worst_cost);
                    }
                }
            }
        }
    }

    (optimal_cost, worst_cost)
}

fn get_boss_stats(input: &str) -> (i32, i32, i32) {
    let mut lines = input.lines();
    let hp = lines
        .next()
        .map(|l| l.split(':').nth(1).unwrap().trim().parse::<i32>().unwrap())
        .unwrap();
    let damage = lines
        .next()
        .map(|l| l.split(':').nth(1).unwrap().trim().parse::<i32>().unwrap())
        .unwrap();
    let armor = lines
        .next()
        .map(|l| l.split(':').nth(1).unwrap().trim().parse::<i32>().unwrap())
        .unwrap();
    (hp, damage, armor)
}

#[aoc(day21, part1)]
pub fn run(input: &str) -> String {
    let boss = get_boss_stats(input);

    let res = find_optimal_equipment(boss).0.to_string();

    utils::solve(2015, 21, "1", &res);

    res
}

#[aoc(day21, part2)]
pub fn run_pt2(input: &str) -> String {
    let boss = get_boss_stats(input);

    let res = find_optimal_equipment(boss).1.to_string();

    utils::solve(2015, 21, "2", &res);

    res
}

#[test]
fn test_player_wins_fight() {
    assert_eq!(player_wins_fight(8, 5, 5, 12, 7, 2), true);
}
