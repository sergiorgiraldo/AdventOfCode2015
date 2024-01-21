extern crate regex;
use regex::Regex;
use std::cmp::max;
use std::iter;
use crate::utils;
use lazy_static::lazy_static;

const GRID_SIZE: (usize, usize) = (1_000, 1_000);

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
}

enum Switch {
    On,
    Off,
    Toggle,
}

struct Instruction {
    operation: Switch,
    from: (usize, usize),
    to: (usize, usize),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let caps: Vec<_> = INSTRUCTION_REGEX
            .captures(s)
            .expect("Cannot parse one of the instructions")
            .iter()
            .map(|c| c.expect("Cannot parse one of the instructions").as_str())
            .skip(1)
            .collect();

        let operation = match caps[0] {
            "turn on" => Switch::On,
            "turn off" => Switch::Off,
            _ => Switch::Toggle,
        };
        let from = (
            caps[1].parse::<usize>().unwrap(),
            caps[2].parse::<usize>().unwrap(),
        );
        let to = (
            caps[3].parse::<usize>().unwrap(),
            caps[4].parse::<usize>().unwrap(),
        );

        Self {
            operation,
            from,
            to,
        }
    }
}

fn switch_lights(grid: &mut Vec<bool>, instructions: &[Instruction]) {
    for instruction in instructions {
        for x in instruction.from.0..=instruction.to.0 {
            for y in instruction.from.1..=instruction.to.1 {
                match instruction.operation {
                    Switch::On => grid[x + y * GRID_SIZE.0] = true,
                    Switch::Off => grid[x + y * GRID_SIZE.0] = false,
                    Switch::Toggle => grid[x + y * GRID_SIZE.0] = !grid[x + y * GRID_SIZE.0],
                }
            }
        }
    }
}

fn switch_lights_advanced(grid: &mut Vec<i32>, instructions: &[Instruction]) {
    for instruction in instructions {
        for x in instruction.from.0..=instruction.to.0 {
            for y in instruction.from.1..=instruction.to.1 {
                match instruction.operation {
                    Switch::On => grid[x + y * GRID_SIZE.0] += 1,
                    Switch::Off => {
                        grid[x + y * GRID_SIZE.0] = max(0, grid[x + y * GRID_SIZE.0] - 1)
                    }
                    Switch::Toggle => grid[x + y * GRID_SIZE.0] += 2,
                }
            }
        }
    }
}

fn make_grid() -> Vec<bool> {
    iter::repeat(false)
        .take(GRID_SIZE.0 * GRID_SIZE.1)
        .collect::<Vec<_>>()
}

#[aoc(day6, part1)]
pub fn run(input: &str) -> String {
    let mut grid = make_grid();

    let instructions: Vec<Instruction> =
        input.lines().map(|l| Instruction::from_str(l)).collect();

    switch_lights(&mut grid, &instructions);

    let res = grid.iter().filter(|c| **c).count().to_string();

    utils::save_answer(&res, "day6.1");

    res
}

#[aoc(day6, part2)]
pub fn run_pt2(input: &str) -> String {
    let mut grid = make_grid().iter().map(|_| 0).collect::<Vec<i32>>();

    let instructions: Vec<Instruction> =
        input.lines().map(|l| Instruction::from_str(l)).collect();

    switch_lights_advanced(&mut grid, &instructions);

    let res = grid.iter().sum::<i32>().to_string();

    utils::save_answer(&res, "day6.2");

    res
}

#[test]
fn test_run() {
    let mut grid = make_grid();
    switch_lights(
        &mut grid,
        &[Instruction::from_str("turn on 0,0 through 999,999")],
    );
    assert_eq!(grid.iter().filter(|c| **c).count(), 1_000_000);

    let mut grid = make_grid();
    switch_lights(
        &mut grid,
        &[Instruction::from_str("toggle 0,0 through 999,0")],
    );
    assert_eq!(grid.iter().filter(|c| **c).count(), 1_000);

    let mut grid = make_grid();
    switch_lights(
        &mut grid,
        &[Instruction::from_str("turn off 499,499 through 500,500")],
    );
    assert_eq!(grid.iter().filter(|c| **c).count(), 0);
}

#[test]
fn test_run_pt2() {
    let mut grid = make_grid().iter().map(|_| 0).collect::<Vec<i32>>();
    switch_lights_advanced(
        &mut grid,
        &[Instruction::from_str("turn on 0,0 through 0,0")],
    );
    assert_eq!(grid.iter().sum::<i32>(), 1);

    let mut grid = make_grid().iter().map(|_| 0).collect::<Vec<i32>>();
    switch_lights_advanced(
        &mut grid,
        &[Instruction::from_str("toggle 0,0 through 999,999")],
    );
    assert_eq!(grid.iter().sum::<i32>(), 2_000_000);
}
