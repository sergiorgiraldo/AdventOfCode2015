use std::collections::HashMap;
use std::iter::FromIterator;
extern crate rand;
use rand::prelude::*;
extern crate rayon;
use rayon::prelude::*;
use crate::utils;
use lazy_static::lazy_static;

const BEST_OF: i32 = 1_000_000;

lazy_static! {
    static ref SPELL_COST: HashMap<Spell, i32> = HashMap::from_iter(vec![
        (Spell::MagicMissile, 53),
        (Spell::Drain, 73),
        (Spell::Shield, 113),
        (Spell::Poison, 173),
        (Spell::Recharge, 229)
    ]);
    static ref ALL_SPELLS: Vec<Spell> = vec![
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Shield,
        Spell::Poison,
        Spell::Recharge
    ];
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

enum Turn {
    Player,
    Boss,
}

fn fight(
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    boss_dmg: i32,
    hard_mode: bool,
) -> Option<i32> {
    let mut player_hp = player_hp;
    let mut player_mana = player_mana;
    let mut boss_hp = boss_hp;

    let mut mana_used = 0;

    let mut turn = Turn::Player;

    let mut shield_active = false;
    let mut shield_count = 0;

    let mut poison_active = false;
    let mut poison_count = 0;

    let mut recharge_active = false;
    let mut recharge_count = 0;

    let mut player_armor;

    loop {
        if shield_active {
            player_armor = 7;
            shield_count += 1;

            if shield_count == 6 {
                shield_active = false;
            }
        } else {
            player_armor = 0;
        }

        if poison_active {
            boss_hp -= 3;
            if boss_hp <= 0 {
                return Some(mana_used);
            }
            poison_count += 1;

            if poison_count == 6 {
                poison_active = false;
            }
        }

        if recharge_active {
            player_mana += 101;
            recharge_count += 1;

            if recharge_count == 5 {
                recharge_active = false;
            }
        }

        match turn {
            Turn::Player => {
                if hard_mode {
                    player_hp -= 1;
                    if player_hp <= 0 {
                        return None;
                    }
                }

                let mut all_spells = ALL_SPELLS.clone();
                all_spells.shuffle(&mut rand::thread_rng());

                let mut spell = None;

                for s in all_spells {
                    if player_mana >= SPELL_COST[&s] {
                        if s == Spell::Shield && shield_active {
                            continue;
                        }
                        if s == Spell::Poison && poison_active {
                            continue;
                        }
                        if s == Spell::Recharge && recharge_active {
                            continue;
                        }

                        spell = Some(s);
                        break;
                    }
                }

                spell.as_ref()?;
                let spell = spell.unwrap();

                player_mana -= SPELL_COST[&spell];
                if player_mana < 0 {
                    player_mana = 0;
                }

                mana_used += SPELL_COST[&spell];

                match spell {
                    Spell::MagicMissile => {
                        boss_hp -= 4;
                        if boss_hp <= 0 {
                            return Some(mana_used);
                        }
                    }
                    Spell::Drain => {
                        boss_hp -= 2;
                        if boss_hp <= 0 {
                            return Some(mana_used);
                        }
                        player_hp += 2
                    }
                    Spell::Shield => {
                        shield_active = true;
                        shield_count = 0;
                    }
                    Spell::Poison => {
                        poison_active = true;
                        poison_count = 0;
                    }
                    Spell::Recharge => {
                        recharge_active = true;
                        recharge_count = 0;
                    }
                }

                turn = Turn::Boss;
            }
            _ => {
                let mut dmg = boss_dmg - player_armor;
                if dmg < 1 {
                    dmg = 1;
                }

                player_hp -= dmg;

                if player_hp <= 0 {
                    return None;
                }

                turn = Turn::Player;
            }
        }
    }
}

fn get_boss_stats(input: &str) -> (i32, i32) {
    let mut lines = input.lines();
    let hp = lines
        .next()
        .map(|l| l.split(':').nth(1).unwrap().trim().parse::<i32>().unwrap())
        .unwrap();
    let damage = lines
        .next()
        .map(|l| l.split(':').nth(1).unwrap().trim().parse::<i32>().unwrap())
        .unwrap();
    (hp, damage)
}

#[aoc(day22, part1)]
pub fn run(input: &str) -> String {
    let boss = get_boss_stats(input);

    let res = (0..BEST_OF)
        .collect::<Vec<i32>>()
        .par_iter()
        .filter_map(|_| fight(50, 500, boss.0, boss.1, false))
        .min()
        .expect("Could not win any fight")
        .to_string();

    utils::solve(2015, 22, "1", &res);

    res    
}

#[aoc(day22, part2)]
pub fn run_pt2(input: &str) -> String {
    let boss = get_boss_stats(input);

    let res = (0..BEST_OF)
        .collect::<Vec<i32>>()
        .par_iter()
        .filter_map(|_| fight(50, 500, boss.0, boss.1, true))
        .min()
        .expect("Could not win any fight")
        .to_string();

    utils::solve(2015, 22, "2", &res);

    res   
}

#[test]
fn test() {
    assert_eq!(
        (0..BEST_OF)
        .collect::<Vec<i32>>()
        .par_iter()
        .filter_map(|_| fight(10, 250, 13, 8, false))
        .min()
        .expect("played Poison + Magic Missile"), 
        226
    );

    assert_eq!(
        (0..BEST_OF)
        .collect::<Vec<i32>>()
        .par_iter()
        .filter_map(|_| fight(10, 250, 14, 8, false))
        .min()
        .expect("played Recharge + Shield + Drain + Poison + Magic Missile"), 
        641
    );
}