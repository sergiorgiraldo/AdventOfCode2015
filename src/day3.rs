use std::collections::HashSet;
use std::ops::AddAssign;
use std::path::Path;
use std::fs;
use std::env;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Location(i32, i32);

impl From<char> for Location {
    fn from(c: char) -> Self {
        match c {
            '^' => Self(0, -1),
            'v' => Self(0, 1),
            '>' => Self(1, 0),
            '<' => Self(-1, 0),
            _ => Self(0, 0),
        }
    }
}

impl AddAssign for Location {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

fn visited_locations(input: &str) -> HashSet<Location> {
    let mut location = Location(0, 0);

    let mut visited_locations = HashSet::new();
    visited_locations.insert(Location(0, 0));

    for step in input.chars().into_iter() {
        let direction = Location::from(step);

        location += direction;

        visited_locations.insert(location);
    }

    visited_locations
}

fn santa_deliver(input: &str) -> usize {
    visited_locations(input).len()
}

fn santa_robo_deliver(input: &str) -> usize {
    let santa_instructions: String = input.chars().step_by(2).collect();
    let robo_instructions: String = input.chars().skip(1).step_by(2).collect();

    let mut santa_locations = visited_locations(&santa_instructions);
    let robo_locations = visited_locations(&robo_instructions);

    santa_locations.extend(robo_locations);

    santa_locations.len()
}

#[aoc(day3, part1)]
pub fn run(input: &str) -> String {
    let res = santa_deliver(input).to_string();

    save_answer(&res, "day3.1");

    res 
}

#[aoc(day3, part2)]
pub fn run_pt2(input: &str) -> String {
    let res = santa_robo_deliver(input).to_string();

    save_answer(&res, "day3.2");

    res
}

pub fn save_answer(ans: &str, part: &str){
    let ans_path = get_current_working_dir();
    let ans_path = Path::new(&ans_path).parent().unwrap().parent().unwrap().parent().unwrap().join("ans");
    let file_path = ans_path.join(format!("{}.txt", part));
    fs::write(file_path, ans).expect("Unable to write file");
}

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string()
    }
}

#[test]
fn test_run() {
    assert_eq!(santa_deliver(">"), 2);
    assert_eq!(santa_deliver("^>v<"), 4);
    assert_eq!(santa_deliver("^v^v^v^v^v"), 2);
}

#[test]
fn test_run_pt2() {
    assert_eq!(santa_robo_deliver("^v"), 3);
    assert_eq!(santa_robo_deliver("^>v<"), 3);
    assert_eq!(santa_robo_deliver("^v^v^v^v^v"), 11);
}
