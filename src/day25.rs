use crate::utils;
use regex::Regex;

fn code_at(row: i128, col: i128) -> i128 {
    let mut diagonal = 2;
    let mut last = 20_151_125;

    loop {
        let mut r = diagonal;
        let mut c = 1;

        while c <= diagonal {
            last = (last * 252_533) % 33_554_393;

            if r == row && c == col {
                return last;
            }

            r -= 1;
            c += 1;
        }
        diagonal += 1;
    }
}

fn get_instructions(input: &str) -> Vec<i128> {
    let re = Regex::new(r"row (\d+), column (\d+)").unwrap();

    let caps = re.captures(input).expect("No matches found");

    let row: u32 = caps.get(1).unwrap().as_str().parse().expect("Not a valid number");
    let column: u32 = caps.get(2).unwrap().as_str().parse().expect("Not a valid number");

    vec![row as i128, column as i128]
}

#[aoc(day25, part1)]
pub fn run(input: &str) -> String {
    let instructions = get_instructions(input);
    
    let res = code_at(instructions[0], instructions[1]).to_string();

    utils::solve(2015, 25, "1", &res);

    res
}