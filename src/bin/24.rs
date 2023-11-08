// use advent_of_code::debug;
use std::collections::HashSet;
use itertools::Itertools;

fn parse(input: &str) -> Vec<u32> {
    let mut res = input.trim().split("\n")
        .map(|line| line.parse::<u32>().unwrap())
        .collect_vec();
    res.reverse(); res
    // res
}

fn get_score(group: &Vec<u32>) -> u64 {
    group.iter().fold(1u64, |acc, e| acc * *e as u64)
}

type G = Vec<Vec<u32>>;
fn is_valid(groups: G) -> bool {
    let mut prev_score = 0u64;
    for group in groups.iter().filter(|g| g.len() != 0) {
        let score = get_score(&group);
        if prev_score > score {
            return false
        }
        prev_score = score;
    }
    true
}

fn grouping(nums: &Vec<u32>, group_amnt: u32) -> Vec<G> {
    fn helper(
        nums: &Vec<u32>,
        target: u32,
        group_idx: usize,
        groups: &mut G,
        target_sum: u32,
        target_idx: usize,
        result: &mut Vec<G>,
        found: &mut HashSet<Vec<u32>>
    ) {
        // debug!(groups);
        if group_idx > 0 && found.contains(&groups[0]) { return; }

        // magic number here, to decrease amount of candidate at first n group
        if group_idx < target_idx && groups[group_idx].len() > 6 { return }

        if group_idx == target_idx {
            groups[group_idx] = nums.clone();
            if is_valid(groups.clone()) {
                // debug!(groups);
                result.push(groups.clone());
                found.insert(groups[0].clone());
            }
            return;
        }

        for i in 0..nums.len() {
            let mut new_nums = nums.clone();
            let popped = new_nums.remove(i);
            if groups[group_idx].len() > 0 && popped > *groups[group_idx].last().unwrap() { continue; }
            if popped < target {
                groups[group_idx].push(popped);
                helper(&new_nums, target - popped,
                    group_idx, groups,
                    target_sum, target_idx, result, found);
                groups[group_idx].pop();
            } else if popped == target {
                groups[group_idx].push(popped);
                if (
                    group_idx > 0
                    && !found.contains(&groups[0])
                    && get_score(&groups[group_idx-1]) < get_score(&groups[group_idx])
                 ) || (
                     group_idx == 0
                     && !found.contains(&groups[0])
                    && groups[0].len() <= 6
                ) {
                    helper(&new_nums, target_sum,
                        group_idx + 1, groups,
                        target_sum, target_idx, result, found);
                }
                groups[group_idx].pop();
            }
        }
    }
    let total_sum = nums.iter().sum::<u32>();
    let target_sum = total_sum / group_amnt;
    // let mut found: HashSet<Vec<u32>> = HashSet::new();
    let mut result: Vec<G> = vec![];

    let mut start_group = vec![];
    for _ in 0..group_amnt {
        start_group.push(vec![]);
    }

    helper(
        nums, target_sum,
        0, &mut start_group,
        target_sum, (group_amnt - 1) as usize, &mut result, &mut HashSet::new());
    result
}


fn find_min(grouped: Vec<G>) -> u64 {
    let mut min_len = usize::MAX;
    for group in &grouped {
        if group[0].len() < min_len { min_len = group[0].len(); }
    }

    let mut min: u64 = u64::MAX;
    for group in grouped.iter().filter(|gs| gs[0].len() == min_len) {
        let prod: u64 = group[0].iter().fold(1u64, |acc, e| acc * *e as u64);
        if prod < min { min = prod; }
    }
    min
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = parse(input);
    let res = grouping(&data, 3);
    Some(find_min(res))
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse(input);
    let res = grouping(&data, 4);
    Some(find_min(res))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input).unwrap(), 99);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input).unwrap(), 44);
    }
}
