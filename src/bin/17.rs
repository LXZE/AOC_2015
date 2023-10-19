// use advent_of_code::debug;
use itertools::Itertools;

fn find_combinations(target: u32, list: &Vec<u32>) -> Vec<Vec<u32>> {
    fn backtracking(
        avail_list: &Vec<u32>,
        target: u32,
        start: usize,
        current_combination: Vec<u32>,
        result: &mut Vec<Vec<u32>>
    ) {
        if target == 0 {
            result.push(current_combination.clone());
            return;
        }

        for idx in start..avail_list.len() {
            if avail_list[idx] <= target {
                let mut new_combination = current_combination.clone();
                new_combination.push(avail_list[idx]);
                backtracking(
                    avail_list,
                    target - avail_list[idx],
                    idx+1,
                    new_combination,
                    result
                );
            }
        }
    }

    let mut result: Vec<Vec<u32>> = vec![];
    backtracking(list, target, 0, vec![], &mut result);
    result
} 

fn generate_data(input: &str) -> Vec<Vec<u32>> {
    let target = if cfg!(test) { 25 } else { 150 };
    let nums = input.trim_end().split("\n")
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec();
    find_combinations(target, &nums)
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = generate_data(input);
    // debug!(result);
    Some(result.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = generate_data(input);
    let mut min = std::usize::MAX;
    result.iter().for_each(|combi| {
        min = std::cmp::min(min, combi.len());
    });
    Some(result.iter().filter(|combi| combi.len() == min).count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input).unwrap(), 4);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input).unwrap(), 3);
    }
}
