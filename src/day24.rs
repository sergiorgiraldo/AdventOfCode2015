use crate::utils;
extern crate itertools;
use itertools::{min, Itertools};

fn compute_qe(presents: &[u128], num_groups: u128) -> u128 {
    let presents_sum: u128 = presents.iter().sum::<u128>();
    let group_size: u128 = presents_sum / num_groups;

    for i in 0..group_size {
        let qes: Vec<u128> = presents
            .iter()
            .cloned()
            .combinations(i as usize)
            .filter(|c| c.iter().sum::<u128>() == group_size)
            .map(|c| c.iter().product())
            .collect();

        if !qes.is_empty() {
            return min(qes).unwrap();
        }
    }

    0
}

#[aoc(day24, part1)]
pub fn run(input: &str) -> String {
    let presents: Vec<u128> = input.lines().map(|l| l.parse::<u128>().unwrap()).collect();

    let res = compute_qe(&presents, 3).to_string();

    utils::solve(2015, 24, "1", &res);

    res
}

#[aoc(day24, part2)]
pub fn run_pt2(input: &str) -> String {
    let presents: Vec<u128> = input.lines().map(|l| l.parse::<u128>().unwrap()).collect();

    let res = compute_qe(&presents, 4).to_string();

    utils::solve(2015, 24, "2", &res);

    res
}

#[test]
fn test() {
    assert_eq!(compute_qe(&[1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 3), 99);
}
