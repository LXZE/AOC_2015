// use advent_of_code::debug;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"(\d+)").unwrap();
}

fn get_row_and_col(input: &str) -> (u32, u32) {
    let x = NUM_REGEX.captures_iter(input)
        .map(|captured| {
            captured.extract::<1>().0.parse::<u32>().unwrap()
        })
        .collect_vec();
    (x[0], x[1])
}

fn get_pos_value(r: u32, c: u32) -> u32 {
    let mut adder = c;
    let mut result = c*(c+1)/2;
    for _ in 1..r {
        result += adder;
        adder += 1;
        // debug!(result);
        // debug!(adder);
    }
    result
}

fn solve(val: u32) -> u32 {
    let mut tmp = 20151125u128;
    for _ in 1..val {
        tmp *= 252533;
        tmp %= 33554393;
    }
    tmp as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let (r, c) = get_row_and_col(input);
    Some(solve(get_pos_value(r, c)))
}

pub fn part_two(_input: &str) -> Option<u32> {
    // day 25 = no part two, :D
    println!("Merry x'mas 2015");
    Some(2015)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let input = advent_of_code::read_file("examples", 25);
        // assert_eq!(part_one("row 1, column 1"), None);
        // assert_eq!(part_one("row 1, column 4"), None);
        // assert_eq!(part_one("row 4, column 1"), None);
        // assert_eq!(part_one("row 4, column 4"), None);
        assert_eq!(part_one("row 6, column 6").unwrap(), 27995004);
    }

    #[test]
    fn test_part_two() {
        // let input = advent_of_code::read_file("examples", 25);
        // assert_eq!(part_two(&input), None);
    }
}
