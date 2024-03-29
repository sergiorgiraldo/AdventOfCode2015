use std::collections::HashMap;
use crate::utils;
use std::iter::FromIterator;

#[derive(Debug, Hash, Eq, PartialEq)]
enum Register {
    A,
    B,
}

impl Register {
    fn from_str(r: &str) -> Self {
        if r.trim_matches(',') == "a" {
            Register::A
        } else {
            Register::B
        }
    }
}

/*
`hlf r` sets register r to half its current value, then continues with the next instruction.
`tpl r`` sets register r to triple its current value, then continues with the next instruction.
`inc r`` increments register r, adding 1 to it, then continues with the next instruction.
`jmp offset` is a jump; it continues with the instruction offset away relative to itself.
`jie r, offset` is like jmp, but only jumps if register r is even ("jump if even").
`jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
*/
#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i64),
    Jie(Register, i64),
    Jio(Register, i64),
    Err,
}

fn parse_program(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let words: Vec<&str> = l.split_whitespace().collect();
            match words[0] {
                "hlf" => Instruction::Hlf(Register::from_str(words[1])),
                "tpl" => Instruction::Tpl(Register::from_str(words[1])),
                "inc" => Instruction::Inc(Register::from_str(words[1])),
                "jmp" => Instruction::Jmp(words[1].parse::<i64>().unwrap()),
                "jie" => Instruction::Jie(
                    Register::from_str(words[1]),
                    words[2].parse::<i64>().unwrap(),
                ),
                "jio" => Instruction::Jio(
                    Register::from_str(words[1]),
                    words[2].parse::<i64>().unwrap(),
                ),
                _ => Instruction::Err,
            }
        })
        .collect()
}

fn execute_program(input: &str, initial_a_value: i64, initial_b_value: i64) -> (i64, i64) {
    let program_instructions = parse_program(input);

    let mut registers: HashMap<Register, i64> = HashMap::from_iter(vec![
        (Register::A, initial_a_value),
        (Register::B, initial_b_value),
    ]);

    let mut curr_instruction: i64 = 0;

    loop {
        let instruction = &program_instructions[curr_instruction as usize];

        let mut new_instruction: i64 = curr_instruction;

        match instruction {
            Instruction::Hlf(reg) => {
                *registers.get_mut(reg).unwrap() = registers[reg] / 2;
                new_instruction += 1;
            }
            Instruction::Tpl(reg) => {
                *registers.get_mut(reg).unwrap() = registers[reg] * 3;
                new_instruction += 1;
            }
            Instruction::Inc(reg) => {
                *registers.get_mut(reg).unwrap() = registers[reg] + 1;
                new_instruction += 1;
            }
            Instruction::Jmp(ins) => {
                new_instruction += *ins;
            }
            Instruction::Jie(reg, ins) => {
                if registers[reg] % 2 == 0 {
                    new_instruction += *ins;
                } else {
                    new_instruction += 1;
                }
            }
            Instruction::Jio(reg, ins) => {
                if registers[reg] == 1 {
                    new_instruction += *ins;
                } else {
                    new_instruction += 1;
                }
            }
            _ => {}
        }

        if new_instruction < 0 || new_instruction >= program_instructions.len() as i64 {
            break;
        }

        curr_instruction = new_instruction;
    }

    (registers[&Register::A], registers[&Register::B])
}

#[aoc(day23, part1)]
pub fn run(input: &str) -> String {
    let res = execute_program(&input, 0, 0).1.to_string(); //register B

    utils::solve(2015, 23, "1", &res);

    res
}

#[aoc(day23, part2)]
pub fn run_pt2(input: &str) -> String {
    let res = execute_program(&input, 1, 0).1.to_string(); //register B

    utils::solve(2015, 23, "2", &res);

    res
}

#[test]
fn test() {
    assert_eq!(
        execute_program(
            r#"inc a
jio a, +2
tpl a
inc a"#,
            0,
            0
        ).0, //register A
        2
    );
}
