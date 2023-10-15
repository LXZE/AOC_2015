use fancy_regex::Regex;

fn is_nice_part_one(str: &str) -> bool {
    let regex_vowel = Regex::new(r"[aeiou]").unwrap();
    if regex_vowel.find_iter(str).count() < 3 { return false }

    let regex_twice = Regex::new(r"(.)\1").unwrap();
    if !regex_twice.is_match(str).unwrap() { return false }

    let regex_invalid = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    if regex_invalid.is_match(str).unwrap() { return false }
    true
}

fn is_nice_part_two(str: &str) -> bool {
    let regex_pair = Regex::new(r"(..).*\1").unwrap();
    if !regex_pair.is_match(str).unwrap() { return false }

    let regex_repeat = Regex::new(r"(.).\1").unwrap();
    if !regex_repeat.is_match(str).unwrap() { return false }
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
