use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use regex::Regex;
// use advent_of_code::debug;

#[derive(Debug)]
struct Stmt {
    val: i32,
    pair: (String, String),
}

fn parse_sentence(str: &str) -> Stmt {
    let sentence_regex = Regex::new(
        r#"(?<from>\w+) would (?<ops>\w+) (?<val>\d+) happiness units by sitting next to (?<to>\w+)."#
    ).unwrap();
    let captured = sentence_regex.captures(str).unwrap();
    let ops = match captured.name("ops").unwrap().as_str() {
        "gain" => |e: i32| e,
        "lose" => |e: i32| -e,
        _ => panic!("line doesn't contain a word gain or lose")
    };
    Stmt {
        val: ops(captured.name("val").unwrap().as_str().parse::<i32>().unwrap()),
        pair: (
            captured.name("from").unwrap().as_str().to_string(),
            captured.name("to").unwrap().as_str().to_string(),
        ),
    }
}

fn calculate(attendees: &HashSet<String>, val_map: &HashMap<(String, String), i32>) -> i32 {
    let mut max = std::i32::MIN;
    attendees.iter().permutations(attendees.len()).for_each(|perm| {
        let mut tmp_max = 0;
        for idx in 0..perm.len() {
            if idx == perm.len()-1 {
                tmp_max += val_map.get(&(perm[idx].to_string(), perm[0].to_string())).unwrap();
                tmp_max += val_map.get(&(perm[0].to_string(), perm[idx].to_string())).unwrap();
            } else {
                tmp_max += val_map.get(&(perm[idx].to_string(), perm[idx+1].to_string())).unwrap();
                tmp_max += val_map.get(&(perm[idx+1].to_string(), perm[idx].to_string())).unwrap();
            }
        }
        max = std::cmp::max(tmp_max, max);
    });
    max
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut attendees: HashSet<String> = HashSet::new();
    let mut val_map: HashMap<(String, String), i32> = HashMap::new();
    let data: Vec<Stmt> = input.trim_end().split("\n").map(parse_sentence).collect();
    data.iter().for_each(|info| {
        attendees.insert(info.pair.0.clone());
        attendees.insert(info.pair.1.clone());
        val_map.insert((info.pair.0.clone(), info.pair.1.clone()), info.val.clone());
    });
    Some(calculate(&attendees, &val_map) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut attendees: HashSet<String> = HashSet::new();
    attendees.insert("you".to_string());

    let mut val_map: HashMap<(String, String), i32> = HashMap::new();
    let data: Vec<Stmt> = input.trim_end().split("\n").map(parse_sentence).collect();
    data.iter().for_each(|info| {
        attendees.insert(info.pair.0.clone());
        attendees.insert(info.pair.1.clone());
        val_map.insert((info.pair.0.clone(), info.pair.1.clone()), info.val.clone());
        val_map.insert((info.pair.0.clone(), "you".to_string()), 0);
        val_map.insert(("you".to_string(), info.pair.1.clone()), 0);
    });
    Some(calculate(&attendees, &val_map) as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input).unwrap(), 330);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input).unwrap(), 286);
    }
}
