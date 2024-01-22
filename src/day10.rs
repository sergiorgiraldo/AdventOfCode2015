extern crate itertools;
use itertools::Itertools;
use crate::utils;

fn look_and_say(n: String) -> String {
    let sequence: String = n
        .to_string()
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(key, val)| format!("{}{}", val.count(), key))
        .collect();

    sequence
}

fn get_sequence(input: String, range: u32) -> String {
    let mut conway_sequence = input.to_string();

    for _ in 0..range {
        conway_sequence = look_and_say(conway_sequence);
    }

    conway_sequence.to_string().chars().count().to_string()
}

#[aoc(day10, part1)]
pub fn run(input: &str) -> String {
    let res = get_sequence(input.to_string(), 40);

    utils::save_answer(&res, "day10.1");

    res
}

#[aoc(day10, part2)]
pub fn run_pt2(input: &str) -> String {
    let res = get_sequence(input.to_string(), 50);

    utils::save_answer(&res, "day10.2");

    res
}

#[test]
fn test_run() {
    assert_eq!(look_and_say("1".to_string()), "11");
    assert_eq!(look_and_say("11".to_string()), "21");
    assert_eq!(look_and_say("21".to_string()), "1211");
    assert_eq!(look_and_say("111221".to_string()), "312211");
}
