extern crate itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Route<'a> {
    origin: &'a str,
    dest: &'a str
}

fn route_len_range(routes: HashMap<Route, u32>, cities: Vec<&str>) -> (u32, u32) {
    let permutations = get_permutations(cities);
    let mut min_dist = u32::max_value();
    let mut max_dist = 0;
    for p in permutations {
        let mut dist = 0;
        for i in 0..p.len()-1 {
            let c1 = p.get(i).unwrap();
            let c2 = p.get(i+1).unwrap();
            let r = Route{origin: c1, dest: c2};
            dist += *routes.get(&r).unwrap();
        }
        if dist < min_dist { min_dist = dist; }
        if dist > max_dist { max_dist = dist; }
    }
    (min_dist, max_dist)
}

fn get_permutations<T: Clone>(v: Vec<T>) -> Vec<Vec<T>> {
    match v.len() {
        0 | 1 => vec![v],
        2 => {
            let rev0 = v.get(1).unwrap().clone();
            let rev1 = v.get(0).unwrap().clone();
            vec![v, vec![rev0, rev1]]
        },
        _ => {
            let mut permutations = vec![];
            for i in 0..v.len() {
                let mut v2 = v.to_vec();
                v2.swap(0, i);
                let curr = v2.get(0).unwrap().clone();
                v2.remove(0);
                for mut p in get_permutations(v2.to_vec()) {
                    p.insert(0, curr.clone());
                    permutations.push(p);
                }
            }
            permutations
        },
    }
}

fn build_map(input: &str) -> (HashMap<Route, u32>, Vec<&str>) {
    let mut routes: HashMap<Route, u32> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split(" to ").collect();
        let origin = split[0];
        let split: Vec<&str> = split[1].split(" = ").collect();
        let dest = split[0];
        let dist = split[1].parse::<u32>().unwrap();
        routes.insert(Route{origin: origin, dest: dest}, dist);
        routes.insert(Route{origin: dest, dest: origin}, dist);
        cities.insert(origin);
        cities.insert(dest);
    }
    let mut cs = vec![];
    for city in cities {
        cs.push(city);
    }
    (routes, cs)
}

#[aoc(day9, part1)]
pub fn run(input: &str) -> String {
    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);

    let res = Ok::<u32, u32>(range.0).unwrap().to_string();

    utils::save_answer(&res, "day9.1");

    res
}

#[aoc(day9, part2)]
pub fn run_pt2(input: &str) -> String {
    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);

    let res = Ok::<u32, u32>(range.1).unwrap().to_string();

    utils::save_answer(&res, "day9.2");

    res
}

#[test]
fn test_run() {
    let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);

    let res = Ok::<u32, u32>(range.0).unwrap();

    assert_eq!(res, 605);
}

#[test]
fn test_run_pt2() {
    let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);

    let res = Ok::<u32, u32>(range.1).unwrap();

    assert_eq!(res, 982);
}
