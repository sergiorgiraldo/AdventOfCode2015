use std::fmt::{Debug, Error, Formatter};
use crate::utils;
use std::iter;
extern crate itertools;
use itertools::Itertools;

const STEP_COUNT:usize = 100;

#[derive(Clone)]
struct Grid {
    data: Vec<bool>,
    width: usize,
    height: usize,
    stuck_lights: Vec<(i32, i32)>,
}

impl Grid {
    fn new(width: usize, height: usize, stuck_lights: Vec<(i32, i32)>) -> Self {
        let mut data: Vec<bool> = vec![];
        data.extend(iter::repeat(false).take(width * height));

        Self {
            data,
            width,
            height,
            stuck_lights,
        }
    }

    fn from_str(s: &str, stuck_lights: Vec<(i32, i32)>) -> Self {
        let lines: Vec<_> = s.lines().collect();

        Self {
            data: lines.join("").chars().map(|c| c == '#').collect(),
            width: lines[0].len(),
            height: lines.len(),
            stuck_lights,
        }
    }

    fn in_grid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }

    fn get(&self, x: i32, y: i32) -> Option<bool> {
        if !self.in_grid(x, y) {
            return None;
        }

        if self.stuck_lights.contains(&(x, y)) {
            return Some(true);
        }

        let pos = (x + (y * self.width as i32)) as usize;
        self.data.get(pos).cloned()
    }

    fn set(&mut self, x: i32, y: i32, value: bool) {
        let pos = (x + y * self.width as i32) as usize;

        if let Some(cell) = self.data.get_mut(pos) {
            if self.stuck_lights.contains(&(x, y)) {
                *cell = true;
            } else {
                *cell = value;
            }
        }
    }
/*
x -1 0 1  y
   # # # -1
   # o #  0
   # # #  1
*/
    fn neighbours_on(&self, x: i32, y: i32) -> usize {
        vec![
            self.get(x - 1, y - 1),
            self.get(x, y - 1),
            self.get(x + 1, y - 1),
            self.get(x - 1, y),
            self.get(x + 1, y),
            self.get(x - 1, y + 1),
            self.get(x, y + 1),
            self.get(x + 1, y + 1),
        ]
        .into_iter()
        .filter_map(|n| n)
        .filter(|n| n.to_owned())
        .count()
    }

    fn step(&mut self) {
        let mut new_grid = Grid::new(self.width, self.height, self.stuck_lights.clone());

        for y in 0..self.height {
            for x in 0..self.width {
                let x = x as i32;
                let y = y as i32;

                let neighbours_on = self.neighbours_on(x, y);
                
                /*
                A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                */
                if self.get(x, y).unwrap() {
                    new_grid.set(x, y, neighbours_on == 2 || neighbours_on == 3);
                } else {
                    new_grid.set(x, y, neighbours_on == 3);
                }
            }
        }

        *self = new_grid;
    }

    fn on_lights(&self) -> usize {
        self.data.iter().filter(|c| c == &&true).count()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let lines = self
            .data
            .chunks(self.width)
            .map(|chunk| {
                chunk
                    .iter()
                    .cloned()
                    .map(|c| if c { "#" } else { "." })
                    .collect::<String>()
            })
            .join("\n");
        write!(f, "{}", lines)
    }
}

#[aoc(day18, part1)]
pub fn run(input: &str) -> String {
    let mut grid = Grid::from_str(&input, vec![]);

    for _ in 0..STEP_COUNT {
        grid.step();
    }

    let res = grid.on_lights().to_string();

    utils::solve(2015, 18, "1", &res);

    res
}

#[aoc(day18, part2)]
pub fn run_pt2(input: &str) -> String {
    let mut grid = Grid::from_str(&input, vec![(0, 0), (99, 0), (0, 99), (99, 99)]);

    for _ in 0..STEP_COUNT {
        grid.step();
    }

    let res = grid.on_lights().to_string();

    utils::solve(2015, 18, "2", &res);

    res
}

#[test]
fn test_run() {
    const TEST_GRID: &str = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
    let mut grid = Grid::from_str(TEST_GRID, vec![]);
    for _ in 0..4 {
        grid.step();
    }

    assert_eq!(grid.on_lights(), 4);
}

#[test]
fn test_run_pt2() {
    const TEST_GRID: &str = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
    let mut grid = Grid::from_str(TEST_GRID, vec![(0, 0), (5, 0), (0, 5), (5, 5)]);
    for _ in 0..5 {
        grid.step();
    }

    assert_eq!(grid.on_lights(), 17);
}
