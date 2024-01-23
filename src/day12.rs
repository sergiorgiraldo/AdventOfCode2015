extern crate serde_json;
use crate::utils;
use serde_json::value::Value;

fn get_value(value: Value) -> i64 {
    match value {
        Value::Array(vals) => vals.iter().cloned().map(get_value).sum(),
        Value::Object(vals) => vals.values().cloned().map(get_value).sum(),
        Value::Number(val) => val.as_i64().unwrap(),
        _ => 0,
    }
}

fn get_value_without_reds(value: Value) -> i64 {
    match value {
        Value::Array(vals) => vals.iter().cloned().map(get_value_without_reds).sum(),
        Value::Object(vals) => {
            let has_any_red = vals.values().cloned().any(|v| match v {
                Value::String(str_v) => str_v == "red",
                _ => false,
            });

            if has_any_red {
                0
            } else {
                vals.values().cloned().map(get_value_without_reds).sum()
            }
        }
        Value::Number(val) => val.as_i64().unwrap(),
        _ => 0,
    }
}

#[aoc(day12, part1)]
pub fn run(input: &str) -> String {
    let json: Value = serde_json::from_str(input).unwrap();

    let res = get_value(json).to_string();

    utils::save_answer(&res, "day12.1");

    res
}

#[aoc(day12, part2)]
pub fn run_pt2(input: &str) -> String {
    let json: Value = serde_json::from_str(input).unwrap();

    let res = get_value_without_reds(json).to_string();

    utils::save_answer(&res, "day12.2");

    res
}

#[test]
fn test_run() {
    let json: Value = serde_json::from_str(r#"[1,2,3]"#).unwrap();
    assert_eq!(get_value(json), 6);

    let json: Value = serde_json::from_str(r#"{"a":2,"b":4}"#).unwrap();
    assert_eq!(get_value(json), 6);

    let json: Value = serde_json::from_str(r#"[[[3]]]"#).unwrap();
    assert_eq!(get_value(json), 3);

    let json: Value = serde_json::from_str(r#"{"a":{"b":4},"c":-1}"#).unwrap();
    assert_eq!(get_value(json), 3);

    let json: Value = serde_json::from_str(r#"{"a":[-1,1]}"#).unwrap();
    assert_eq!(get_value(json), 0);

    let json: Value = serde_json::from_str(r#"[]"#).unwrap();
    assert_eq!(get_value(json), 0);

    let json: Value = serde_json::from_str(r#"{}"#).unwrap();
    assert_eq!(get_value(json), 0);
}

#[test]
fn test_run_pt2() {
    let json: Value = serde_json::from_str(r#"[1,2,3]"#).unwrap();
    assert_eq!(get_value_without_reds(json), 6);

    let json: Value = serde_json::from_str(r#"[1,{"c":"red","b":2},3]"#).unwrap();
    assert_eq!(get_value_without_reds(json), 4);

    let json: Value = serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).unwrap();
    assert_eq!(get_value_without_reds(json), 0);

    let json: Value = serde_json::from_str(r#"[1,"red",5]"#).unwrap();
    assert_eq!(get_value_without_reds(json), 6);
}
