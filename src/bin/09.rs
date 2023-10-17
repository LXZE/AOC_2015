use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

type Edge<'a> = (&'a str, &'a str, u32);

fn parse_edge(line: &str) -> Edge {
    let edge_regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let captured = edge_regex.captures(line).unwrap();
    (
        captured.get(1).unwrap().as_str(),
        captured.get(2).unwrap().as_str(),
        captured.get(3).unwrap().as_str().parse::<u32>().unwrap(),
    )
}

fn solve(edges: Vec<Edge>) -> Vec<u32> {
    let mut nodes_name = HashSet::new();
    let mut nodes_len_map = HashMap::new();
    for (from, to, weight) in edges {
        nodes_len_map.insert((from, to), weight);
        nodes_len_map.insert((to, from), weight);
        nodes_name.insert(from);
        nodes_name.insert(to);
    }

    let mut result_list = vec![];
    for entry in nodes_name.clone().into_iter().combinations(nodes_name.len()) {
        let mut tmp = 0;
        for idx in 0..entry.len() - 1 {
            let pair = entry[idx..idx + 2].to_vec();
            tmp += nodes_len_map[&(pair[0], pair[1])];
        }
        result_list.push(tmp);
    }
    result_list
}

pub fn part_one(input: &str) -> Option<u32> {
    let edge_list: Vec<Edge> = input.trim_end().split("\n").map(parse_edge).collect();
    Some(solve(edge_list).into_iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let edge_list: Vec<Edge> = input.trim_end().split("\n").map(parse_edge).collect();
    Some(solve(edge_list).into_iter().max().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input).unwrap(), 605);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input).unwrap(), 982);
    }
}
