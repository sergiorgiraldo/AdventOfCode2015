use std::iter::FromIterator;
use crate::utils;

#[aoc(day20, part1)]
pub fn run(input: &str) -> String {
    let mut res:String = "".to_string();

    let input_int: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return "Invalid input".to_string(),
    };

    for house_nr in 1..input_int {
        if presents_at1(house_nr) >= input_int {
            res = house_nr.to_string();
            break
        }
    }

    utils::solve(2015, 20, "1", &res);

    res
}

#[aoc(day20, part2)]
pub fn run_pt2(input: &str) -> String {
    let mut res:String = "".to_string();

    let input_int: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return "Invalid input".to_string(),
    };

    for house_nr in 1..input_int {
        if presents_at2(house_nr) >= input_int {
            res = house_nr.to_string();
            break
        }
    }

    utils::solve(2015, 20, "2", &res);

    res
}

fn presents_at1(house_nr: u64) -> u64 {
    divisors(house_nr)
        .into_iter()
        .sum::<u64>() * 10
}

fn presents_at2(house_nr: u64) -> u64 {
    divisors(house_nr)
        .into_iter()
        .filter(|d| house_nr / d <= 50)
        .sum::<u64>()
        * 11
}

fn divisors(n: u64) -> Vec<u64> {
    let mut small_divisors: Vec<u64> = Vec::from_iter(1..((n as f64).sqrt() as u64 + 1))
        .into_iter()
        .filter(|i| n % *i == 0)
        .collect();

    let mut large_divisors: Vec<u64> = small_divisors
        .iter()
        .filter(|d| n != **d * **d)
        .map(|d| n / d)
        .collect();

    small_divisors.append(&mut large_divisors);
    small_divisors
}

#[test]
fn test_get_house1_presents() {
    assert_eq!(presents_at1(1), 10);
    assert_eq!(presents_at1(2), 30);
    assert_eq!(presents_at1(3), 40);
    assert_eq!(presents_at1(4), 70);
    assert_eq!(presents_at1(5), 60);
    assert_eq!(presents_at1(6), 120);
    assert_eq!(presents_at1(7), 80);
    assert_eq!(presents_at1(8), 150);
    assert_eq!(presents_at1(9), 130);
}
