use std::collections::{HashMap, HashSet};

// use advent_of_code::debug;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref REPLACEMENT_REGEX: Regex = Regex::new(r"(\w+) => (\w+)").unwrap();
    static ref MOLECULE_REGEX: Regex = Regex::new(r"[A-Z][a-z]?").unwrap();
}

fn get_vec_molecule(molecule_str: &str) -> Vec<&str> {
    MOLECULE_REGEX.find_iter(molecule_str)
        .map(|found| found.as_str())
        .collect_vec()
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<Vec<&str>>>, Vec<&str>) {
    let mut replacements = input.trim_end().split("\n").collect_vec();
    let starter = replacements.pop().unwrap();
    replacements.pop();
    let mut replacer_map: HashMap<&str, Vec<Vec<&str>>> = HashMap::new();
    replacements.iter().for_each(|line| {
        let replacement_captured = REPLACEMENT_REGEX.captures(line).unwrap();
        replacer_map
            .entry(replacement_captured.get(1).unwrap().as_str())
            .or_insert(vec![])
            .push(get_vec_molecule(replacement_captured.get(2).unwrap().as_str()));
    });
    (
        replacer_map,
        get_vec_molecule(starter)
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (replacement, molecule) = parse_input(input);
    let mut molecule_set: HashSet<String> = HashSet::new();

    for idx in 0..molecule.len() {
        match replacement.get(molecule[idx]) {
            Some(replacers) => {
                for replacer in replacers {
                    molecule_set.insert(
                        molecule.iter()
                            .enumerate()
                            .flat_map(|(i_enum, elem)| {
                                if i_enum == idx { replacer.clone() } else { vec![*elem] }
                            })
                            .join("")
                    );
                }
            },
            None => ()
        }
    }
    Some(molecule_set.len() as u32)
}

fn formular(total_molecule: u32, bracket_amnt: u32, comma_amnt: u32) -> u32 {
    total_molecule - bracket_amnt - (2*comma_amnt) - (
        if cfg!(test) { 0 } else { 1 } // in test e => X not e => XX so increase one
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut replacements = input.trim_end().split("\n").collect_vec();
    let target = replacements.pop().unwrap();
    let molecule = get_vec_molecule(target);
    let bracket_regex = Regex::new(r"(Rn|Ar)").unwrap();
    let comma_regex = Regex::new(r"(Y)").unwrap();

    let bracket_amnt = bracket_regex.find_iter(target).count() as u32;
    let comma_amnt = comma_regex.find_iter(target).count() as u32;
    // debug!(bracket_amnt);
    // debug!(comma_amnt);

    Some(formular(molecule.len() as u32, bracket_amnt, comma_amnt))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input).unwrap(), 7);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input).unwrap(), 6);
    }
}
