// use advent_of_code::debug;
use std::collections::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref INST_REGEX: Regex = Regex::new(r"^(?<inst>\w{3}) (?<reg>[ab])?((, )?(?<offset>[+-]\d+))?$").unwrap();
}

#[derive(Debug)]
struct Code {
    inst: String,
    register: usize,
    offset: i32,
}

fn parse_input(input: &str) -> Vec<Code> {
    input.trim().split("\n").map(|line| {
        INST_REGEX.captures(line)
        .map(|captured| {
            Code {
                inst: captured.name("inst").unwrap().as_str().to_string(),
                register: match captured.name("reg") {
                    Some(s) => match s.as_str() { "a" => 0, "b" => 1, _ => 2},
                    None => 0,
                },
                offset: match captured.name("offset") {
                    Some(offset_str) => offset_str.as_str().parse::<i32>().unwrap(),
                    None => 0,
                },
            }
        })
        .unwrap()
    }).collect_vec()
}

fn exec(codes: Vec<Code>, start: u32) -> u32 {
    let mut pc: i32 = 0;
    let range = codes.len() as i32;

    let mut reg: HashMap<usize, u32> = HashMap::from([(0, start), (1, 0)]);

    // debug!(codes);
    while pc < range {
        let cmd = &codes[pc as usize];
        match cmd.inst.as_str() {
            "hlf" => {
                if let Some(r) = reg.get_mut(&cmd.register) {
                    *r /= 2;
                    pc += 1;
                };
            },
            "tpl" => {
                if let Some(r) = reg.get_mut(&cmd.register) {
                    *r *= 3;
                    pc += 1;
                };
            },
            "inc" => {
                if let Some(r) = reg.get_mut(&cmd.register) {
                    *r += 1;
                    pc += 1;
                };
            },
            "jmp" => {
                pc += cmd.offset;
            },
            "jie" => {
                if let Some(r) = reg.get(&cmd.register) {
                    if *r % 2 == 0 {
                        pc += cmd.offset;
                    } else {
                        pc += 1;
                    }
                };
            },
            "jio" => {
                if let Some(r) = reg.get(&cmd.register) {
                    if *r == 1 {
                        pc += cmd.offset;
                    } else {
                        pc += 1;
                    }
                };
            },
            _ => panic!("{}", format!("unknown instruction: {}", cmd.inst))
        }
    }
    *reg.get(&1).unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let codes = parse_input(input);
    Some(exec(codes, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    let codes = parse_input(input);
    Some(exec(codes, 1))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(0));
    }
}
