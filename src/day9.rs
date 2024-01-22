extern crate itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils;
use std::cmp::min;
use std::cmp::max;
extern crate permutohedron;
use permutohedron::LexicalPermutation;
#[derive(Debug, PartialEq, Eq, Hash)]
struct Route {
    origin: String,
    dest: String
}

impl Route {
    //London to Dublin = 464
    fn from_str(s: &str, round_trip: bool) -> (Self, u32) {
        let split: Vec<&str> = s.split(" ").collect();
        let origin = split[0].to_string();
        let dest = split[2].to_string();
        let dist = split[4].parse::<u32>().unwrap();

        if round_trip {
            (Route{origin: dest, dest: origin}, dist)
        }
        else{
            (Route{origin, dest}, dist)
        }
    }
}

fn route_len_range(routes: HashMap<Route, u32>, mut cities: Vec<String>) -> (u32, u32) {
    let mut permutations = Vec::new(); //permutohedron
    let mut min_dist = u32::max_value();
    let mut max_dist = 0;

    loop {
        permutations.push(cities.to_vec());
        if !cities.next_permutation() {
            break;
        }
    }

    for permutation in permutations {
        let mut dist = 0;

        for i in 0..permutation.len()-1 {
            let city_origin = permutation.get(i).unwrap();
            let city_dest = permutation.get(i+1).unwrap();
            let route = Route{origin: city_origin.to_string(), dest: city_dest.to_string()};
            
            dist += *routes.get(&route).unwrap();
        }

        min_dist = min(dist, min_dist);
        max_dist = max(dist, max_dist);
    }

    (min_dist, max_dist)
}

fn build_map(input: &str) -> (HashMap<Route, u32>, Vec<String>) {
    let mut routes: HashMap<Route, u32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();

    input.lines().for_each(|line| {
        let route = Route::from_str(line, false);
        routes.insert(route.0, route.1);
        
        let route = Route::from_str(line, true);
        routes.insert(route.0, route.1);        
    }); 

    for route in routes.keys() {
        cities.insert(route.origin.clone());
    }

    let cs: Vec<String> = cities.iter().cloned().collect();

    (routes, cs)
}

#[aoc(day9, part1)]
pub fn run(input: &str) -> String {
    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);

    let res = range.0.to_string();

    utils::save_answer(&res, "day9.1");

    res
}

#[aoc(day9, part2)]
pub fn run_pt2(input: &str) -> String {
    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);

    let res = range.1.to_string();

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

    let res = range.0;

    assert_eq!(res, 605);
}

#[test]
fn test_run_pt2() {
    let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    let (routes, cities) = build_map(input);
    let range = route_len_range(routes, cities);

    let res = range.1;

    assert_eq!(res, 982);
}
