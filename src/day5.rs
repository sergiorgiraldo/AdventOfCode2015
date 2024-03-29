extern crate itertools;
use crate::utils;
use itertools::Itertools;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const DISALLOWED_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

trait Nice {
    fn is_nice(&self) -> bool;
    fn is_the_nicest(&self) -> bool;
}

impl Nice for String {
    fn is_nice(&self) -> bool {
        self.chars().filter(|c| VOWELS.contains(c)).count() >= 3
            && self
                .chars()
                .group_by(|&c| c)
                .into_iter()
                .any(|(_, vals)| vals.count() > 1)
            && !DISALLOWED_STRINGS.iter().any(|ds| self.contains(ds))
    }

    fn is_the_nicest(&self) -> bool {
        self.chars()
            .enumerate()
            .zip(self.chars().enumerate().skip(1))
            .sorted_by_key(|g| format!("{}{}", (g.0).1, (g.1).1)) // each char is now a tuple with the char and the pos
            .group_by(|g| format!("{}{}", (g.0).1, (g.1).1)) // group by char pairs
            .into_iter()
            .map(|(key, val)| (key, val.collect::<Vec<_>>())) // collect the group values
            .filter(|(_, val)| val.iter().count() > 1) // exclude not repeated char groups
            .filter(|(_, val)| val.len() != 2 || (val[0].1).0 != (val[1].0).0) // exclude consecutive repetitions like "aaa"
            .any(|(_, vals)| vals.len() > 1) // a valid word must have at least two non consecutive repetitions

            && self.chars()
            .zip(self.chars().skip(1))
            .zip(self.chars().skip(2))
            .any(|g| (g.0).0 == g.1)
    }
}

#[aoc(day5, part1)]
pub fn run(input: &str) -> String {
    let res = input
        .lines()
        .filter(|s| s.to_string().is_nice())
        .count()
        .to_string();

    utils::save_answer(&res, "day5.1");

    res
}

#[aoc(day5, part2)]
pub fn run_pt2(input: &str) -> String {
    let res = input
        .lines()
        .filter(|s| s.to_string().is_the_nicest())
        .count()
        .to_string();

    utils::save_answer(&res, "day5.2");

    res
}

#[test]
fn test_run() {
    assert!("ugknbfddgicrmopn".to_string().is_nice());
    assert!("aaa".to_string().is_nice());
    assert!(!"jchzalrnumimnmhp".to_string().is_nice());
    assert!(!"haegwjzuvuyypxyu".to_string().is_nice());
    assert!(!"dvszwmarrgswjxmb".to_string().is_nice());
}

#[test]
fn test_run_pt2() {
    assert!("xyxy".to_string().is_the_nicest());
    assert!(!"aaa".to_string().is_the_nicest());
    assert!(!"aabcdefgaa".to_string().is_the_nicest());
    assert!("qjhvhtzxzqqjkmpb".to_string().is_the_nicest());
    assert!("xxyxx".to_string().is_the_nicest());
    assert!(!"uurcxstgmygtbstg".to_string().is_the_nicest());
    assert!(!"ieodomkazucvgmuy".to_string().is_the_nicest());
    assert!("rxexcbwhiywwwwnu".to_string().is_the_nicest());
}
