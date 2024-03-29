extern crate itertools;
use crate::utils;
use itertools::Itertools;

struct Gift {
    l: u32,
    w: u32,
    h: u32,

    sides: [u32; 3],
}

impl Gift {
    fn smallest_side(&self) -> u32 {
        *self.sides.iter().min().unwrap()
    }
}

impl From<&str> for Gift {
    fn from(s: &str) -> Self {
        let sizes = s.split('x').collect::<Vec<&str>>();
        let l = sizes[0].parse().unwrap();
        let w = sizes[1].parse().unwrap();
        let h = sizes[2].parse().unwrap();

        Self {
            l,
            w,
            h,
            sides: [l * w, w * h, h * l],
        }
    }
}

fn compute_wrapping_paper(input: &str) -> u32 {
    let gifts: Vec<Gift> = input.lines().map(|g| g.into()).collect();

    gifts
        .iter()
        .map(|gift| {
            let sides_sum: u32 = gift.sides.iter().map(|s| s * 2).sum();
            sides_sum + gift.smallest_side()
        })
        .sum()
}

fn compute_ribbon(input: &str) -> u32 {
    let gifts: Vec<Gift> = input.lines().map(|g| g.into()).collect();

    gifts
        .iter()
        .map(|gift| {
            let dimensions = [gift.l, gift.w, gift.h];
            let smallest_dimensions_sum: u32 =
                dimensions.iter().sorted().take(2).map(|d| d * 2).sum();
            let all_dimensions_sum: u32 = dimensions.iter().product();

            smallest_dimensions_sum + all_dimensions_sum
        })
        .sum()
}

#[aoc(day2, part1)]
pub fn run(input: &str) -> String {
    let res = compute_wrapping_paper(input).to_string();

    utils::save_answer(&res, "day2.1");

    res
}

#[aoc(day2, part2)]
pub fn run_pt2(input: &str) -> String {
    let res = compute_ribbon(input).to_string();

    utils::save_answer(&res, "day2.2");

    res
}

#[test]
fn test_run() {
    assert_eq!(compute_wrapping_paper("2x3x4"), 58);
    assert_eq!(compute_wrapping_paper("1x1x10"), 43);
}

#[test]
fn test_run_pt2() {
    assert_eq!(compute_ribbon("3x2x4"), 34);
    assert_eq!(compute_ribbon("1x1x10"), 14);
}
