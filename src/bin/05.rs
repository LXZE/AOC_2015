use fancy_regex::Regex;

use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_VOWEL: Regex = Regex::new(r"[aeiou]").unwrap();
    static ref REGEX_TWICE: Regex = Regex::new(r"(.)\1").unwrap();
    static ref REGEX_INVALID: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    static ref REGEX_PAIR: Regex = Regex::new(r"(..).*\1").unwrap();
    static ref REGEX_REPEAT: Regex = Regex::new(r"(.).\1").unwrap();
}

fn is_nice_part_one(str: &str) -> bool {
    if REGEX_VOWEL.find_iter(str).count() < 3 { return false }
    if !REGEX_TWICE.is_match(str).unwrap() { return false }
    if REGEX_INVALID.is_match(str).unwrap() { return false }
    true
}

fn is_nice_part_two(str: &str) -> bool {
    if !REGEX_PAIR.is_match(str).unwrap() { return false }
    if !REGEX_REPEAT.is_match(str).unwrap() { return false }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input.trim_end().split("\n").fold(0, |acc, line| {
        acc + if is_nice_part_one(line) { 1 } else { 0 }
    });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input.trim_end().split("\n").fold(0, |acc, line| {
        acc + if is_nice_part_two(line) { 1 } else { 0 }
    });
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        // let lines = input.trim_end().split("\n").collect::<Vec<&str>>();
        assert_eq!(part_one(&input).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        // let input = advent_of_code::read_file("examples", 5);
        assert_eq!(is_nice_part_two("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_part_two("xxyxx"), true);
        assert_eq!(is_nice_part_two("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_part_two("ieodomkazucvgmuy"), false);
    }
}
