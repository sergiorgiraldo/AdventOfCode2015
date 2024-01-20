use std::path::Path;
use std::fs;
use std::env;

const UP_CHAR: &char = &'(';
const DOWN_CHAR: &char = &')';

fn compute_floor(input: &str) -> i32 {
    let ups = input.chars().filter(|c| c == UP_CHAR).count() as i32;
    let downs = input.chars().filter(|c| c == DOWN_CHAR).count() as i32;

    ups - downs
}

fn find_basement_step(input: &str) -> usize {
    let mut current_floor: i32 = 0;

    for (idx, c) in input.chars().enumerate() {
        current_floor += if c == *UP_CHAR { 1 } else { -1 };
        if current_floor == -1 {
            return idx + 1;
        }
    }

    0
}

#[aoc(day1, part1)]
fn run(input: &str) -> String {
    let res = compute_floor(input).to_string();

    save_answer(&res, "day1.1");

    res
}

#[aoc(day1, part2)]
fn run_pt2(input: &str) -> String {
    let res = find_basement_step(input).to_string();

    save_answer(&res, "day1.2");

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
    assert_eq!(compute_floor("(())"), 0);
    assert_eq!(compute_floor("()()"), 0);
    assert_eq!(compute_floor("((("), 3);
    assert_eq!(compute_floor("(()(()("), 3);
    assert_eq!(compute_floor("))((((("), 3);
    assert_eq!(compute_floor("())"), -1);
    assert_eq!(compute_floor("())"), -1);
    assert_eq!(compute_floor("))("), -1);
    assert_eq!(compute_floor(")))"), -3);
    assert_eq!(compute_floor(")))"), -3);
    assert_eq!(compute_floor(")())())"), -3);
}

#[test]
fn test_run_pt2() {
    assert_eq!(find_basement_step(")"), 1);
    assert_eq!(find_basement_step("()())"), 5);
}
