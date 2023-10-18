use std::collections::HashMap;

// use advent_of_code::debug;
use itertools::Itertools;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref THINGS: HashMap<&'static str, u32> = HashMap::from_iter([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    static ref LINE_REGEX: Regex = Regex::new(
        r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)"
    ).unwrap();
    static ref GT: Vec<&'static str> = vec!["cats", "trees"];
    static ref LT: Vec<&'static str> = vec!["pomeranians", "goldfish"];
}

fn parse_line(line: &str) -> (u32, Vec<(String, u32)>) {
    let captured = LINE_REGEX.captures(line).unwrap();
    (
        captured.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        (1..=3).into_iter().map(|i| {
            (
                captured.get(i*2).unwrap().as_str().to_string(),
                captured.get((i*2)+1).unwrap().as_str().parse::<u32>().unwrap(),
            )
        }).collect_vec()
    )
}

fn is_match_property(props: &Vec<(String, u32)>) -> bool {
    props.iter().all(|(key, val)| {
        THINGS.get(key.as_str()).unwrap() == val
    })
}

fn is_match_property_part2(props: &Vec<(String, u32)>) -> bool {
    props.iter().all(|(key, val)| {
        if GT.contains(&key.as_str()) {
            val > THINGS.get(key.as_str()).unwrap()
        } else if LT.contains(&key.as_str()) {
            val < THINGS.get(key.as_str()).unwrap()
        } else {
            THINGS.get(key.as_str()).unwrap() == val
        }
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let sues: Vec<(u32, Vec<(String, u32)>)> = input.trim_end().split("\n").map(parse_line).collect();
    let the_one = sues.iter()
        .find(|&(_, properties)| {
            is_match_property(properties)
        }).unwrap();
    Some(the_one.0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sues: Vec<(u32, Vec<(String, u32)>)> = input.trim_end().split("\n").map(parse_line).collect();
    let the_one = sues.iter()
        .find(|&(_, properties)| {
            is_match_property_part2(properties)
        }).unwrap();
    Some(the_one.0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input).unwrap(), 40);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input).unwrap(), 241);
    }
}
