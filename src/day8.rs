extern crate regex;
use crate::utils;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REPLACE_QUOTE_REGEX: Regex = Regex::new(r#"\\""#).unwrap();
    static ref REPLACE_SLASHES_REGEX: Regex = Regex::new(r"\\\\").unwrap();
    static ref REPLACE_CHAR_REGEX: Regex = Regex::new(r"\\x[0-9a-f]{2}").unwrap(); // ASCII codes
}

fn count_actual_chars(s: &str) -> i32 {
    let s = REPLACE_QUOTE_REGEX.replace_all(&s, r#"""#);
    let s = REPLACE_SLASHES_REGEX.replace_all(&s, r"\");
    let s = REPLACE_CHAR_REGEX.replace_all(&s, "_"); // replace ASCII codes with single character

    s.chars().count() as i32 - 2 // -2 for the quotes
}

fn encode_and_count(s: &str) -> i32 {
    let s = s.replace(r#"\"#, r#"\\"#);
    let s = s.replace(r#"""#, r#"\""#);

    s.chars().count() as i32 + 2 // +2 for the quotes
}

#[aoc(day8, part1)]
pub fn run(input: &str) -> String {
    let actual_chars: i32 = input.lines().map(|l| count_actual_chars(l)).sum();
    let raw_chars: i32 = input.lines().map(|l| l.chars().count() as i32).sum();

    let res = (raw_chars - actual_chars).to_string();

    utils::save_answer(&res, "day8.1");

    res
}

#[aoc(day8, part2)]
pub fn run_pt2(input: &str) -> String {
    let new_string_chars: i32 = input.lines().map(|l| encode_and_count(l)).sum();
    let raw_chars: i32 = input.lines().map(|l| l.chars().count() as i32).sum();

    let res = (new_string_chars - raw_chars).to_string();

    utils::save_answer(&res, "day8.2");

    res
}

#[test]
fn test_run() {
    assert_eq!(count_actual_chars(r#""""#), 0);
    assert_eq!(count_actual_chars(r#""abc""#), 3);
    assert_eq!(count_actual_chars(r#""aaa\"aaa""#), 7);
    assert_eq!(count_actual_chars(r#""\x27""#), 1);
}

#[test]
fn test_run_pt2() {
    assert_eq!(encode_and_count(r#""""#), 6);
    assert_eq!(encode_and_count(r#""abc""#), 9);
    assert_eq!(encode_and_count(r#""aaa\"aaa""#), 16);
    assert_eq!(encode_and_count(r#""\x27""#), 11);
}
