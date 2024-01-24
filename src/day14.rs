use crate::utils;

const RACE_LOOPS: i32 = 2503;

struct Reindeer {
    speed: i32,
    speed_seconds: i32,
    rest: i32,
    speed_cicle: i32,
    rest_cicle: i32,
    distance: i32,
    resting: bool,
    score: i32,
}

impl Reindeer {
    fn from_str(s: &str) -> Self {
        let words: Vec<&str> = s.split_whitespace().collect();
        let speed = words[3].parse::<i32>().unwrap();
        let speed_seconds = words[6].parse::<i32>().unwrap();
        let rest = words[13].parse::<i32>().unwrap();

        Self {
            speed,
            speed_seconds,
            rest,
            speed_cicle: speed_seconds,
            rest_cicle: 0,
            distance: 0,
            resting: false,
            score: 0,
        }
    }

    fn fly(&mut self) {
        if self.resting {
            if self.rest_cicle == 2 {
                self.speed_cicle = self.speed_seconds;
                self.resting = false;
            } else {
                self.rest_cicle -= 1;
            }
        } else if self.speed_cicle == 0 {
            self.rest_cicle = self.rest;
            self.resting = true;
        } else {
            self.speed_cicle -= 1;
            self.distance += self.speed;
        }
    }
}

fn race_with_score(reindeers: &mut Vec<Reindeer>, rounds: i32) -> i32 {
    for _ in 0..rounds {
        for reindeer in reindeers.iter_mut() {
            reindeer.fly();
        }

        reindeers.sort_by_key(|r| -r.distance);
        let top_distance = reindeers[0].distance;

        for reindeer in reindeers.iter_mut() {
            if reindeer.distance == top_distance {
                reindeer.score += 1;
            }
        }
    }

    let winning_reindeer = reindeers.iter().max_by_key(|r| r.score).unwrap();
    
    winning_reindeer.score
}

#[aoc(day14, part1)]
pub fn run(input: &str) -> String {
    let mut reindeers: Vec<_> = input.lines().map(Reindeer::from_str).collect();

    for _ in 0..RACE_LOOPS {
        for reindeer in reindeers.iter_mut() {
            reindeer.fly();
        }
    }

    let winning_reindeer = reindeers.iter().max_by_key(|r| r.distance).unwrap();
    
    let res = winning_reindeer.distance.to_string();

    utils::save_answer(&res, "day14.1");

    res
}

#[aoc(day14, part2)]
pub fn run_pt2(input: &str) -> String {
    let mut reindeers: Vec<_> = input.lines().map(Reindeer::from_str).collect();

    let res = race_with_score(&mut reindeers, RACE_LOOPS).to_string();

    utils::save_answer(&res, "day14.2");

    res
}

#[test]
fn test_run() {
    let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

    let mut reindeers: Vec<_> = input.lines().map(Reindeer::from_str).collect();

    for reindeer in reindeers.iter_mut() {
        reindeer.fly();
    }

    assert_eq!(reindeers[0].distance, 14);
    assert_eq!(reindeers[1].distance, 16);

    for _ in 0..9 {
        for reindeer in reindeers.iter_mut() {
            reindeer.fly();
        }
    }

    assert_eq!(reindeers[0].distance, 140);
    assert_eq!(reindeers[1].distance, 160);

    for reindeer in reindeers.iter_mut() {
        reindeer.fly();
    }

    assert_eq!(reindeers[0].distance, 140);
    assert_eq!(reindeers[1].distance, 176);

    for _ in 0..988 {
        for reindeer in reindeers.iter_mut() {
            reindeer.fly();
        }
    }

    assert_eq!(reindeers[0].distance, 1120);
    assert_eq!(reindeers[1].distance, 1056);
}

#[test]
fn test_run_pt2() {
    let input = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

    let mut reindeers: Vec<_> = input.lines().map(Reindeer::from_str).collect();

    assert_eq!(race_with_score(&mut reindeers, 1000), 689);
}
