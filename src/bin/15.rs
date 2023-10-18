use std::{iter::zip, cmp};

// use advent_of_code::debug;
use itertools::Itertools;
use regex::Regex;

#[allow(dead_code)]
#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn score(&self, proportion: u32) -> Vec<i32> {
        [self.capacity, self.durability, self.flavor, self.texture]
            .iter_mut()
            .map(|p| *p * proportion as i32)
            .collect_vec()
    }

    fn score_with_calories(&self, proportion: u32) -> Vec<i32> {
        let mut result = self.score(proportion);
        result.push(self.calories * proportion as i32);
        result
    }
}

fn parse_ingredient(line: &str) -> Ingredient {
    let line_regex = Regex::new(
        r#"(?<name>\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)"#
    ).unwrap();
    let captured = line_regex.captures(line).unwrap();
    let (capacity, durability, flavor, texture, calories) = match (2..=6).map(|i| {
        captured.get(i).unwrap().as_str().parse::<i32>().unwrap()
    }).collect::<Vec<i32>>()[..] {
        [a,b,c,d,e] => (a,b,c,d,e),
        _ => panic!("unexpected parsed result"),
    };

    Ingredient {
        name: captured.name("name").unwrap().as_str().to_string(),
        capacity, durability, flavor, texture, calories
    }
}

fn find_combinations(sum: i32, n: usize) -> Vec<Vec<i32>> {
    fn find_combinations_helper(
        remaining_sum: i32,
        remaining_count: usize,
        current_combination: Vec<i32>,
        all_combinations: &mut Vec<Vec<i32>>,
    ) {
        if remaining_count == 0 {
            if remaining_sum == 0 {
                all_combinations.push(current_combination);
            }
            return;
        }

        for i in 0..=remaining_sum {
            let mut new_combination = current_combination.clone();
            new_combination.push(i);
            find_combinations_helper(
                remaining_sum - i,
                remaining_count - 1,
                new_combination,
                all_combinations,
            );
        }
    }

    let mut all_combinations = Vec::new();
    find_combinations_helper(sum, n, Vec::new(), &mut all_combinations);
    all_combinations
}

fn generate_data(input: &str) -> (Vec<Ingredient>, Vec<Vec<i32>>) {
    let ingredients: Vec<Ingredient> = input.trim_end().split("\n").map(parse_ingredient).collect();
    let combinations = find_combinations(100, ingredients.len());
    (ingredients, combinations)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ingredients, combinations) = generate_data(input);

    let mut max = std::u32::MIN;
    for combination in combinations {
        let tmp: Vec<i32> = zip(combination, &ingredients).map(|(proportion, ingredient)| {
            ingredient.score(proportion as u32)
        }).reduce(|acc, item| zip(acc,item).map(|(a, b)| a+b ).collect_vec()).unwrap();

        if tmp.iter().any(|ingredient| *ingredient <= 0) { continue; }
        max = cmp::max(max, tmp.iter().product::<i32>() as u32);
    }
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (ingredients, combinations) = generate_data(input);

    let mut max = std::u32::MIN;
    for combination in combinations {
        let mut tmp: Vec<i32> = zip(combination, &ingredients).map(|(proportion, ingredient)| {
            ingredient.score_with_calories(proportion as u32)
        }).reduce(|acc, item| {
            zip(acc,item).map(|(a, b)| a+b ).collect_vec()
        }).unwrap();

        if tmp.iter().any(|ingredient| *ingredient <= 0)
            || *tmp.last().unwrap() != 500 { continue; }
        tmp.pop();
        max = cmp::max(max, tmp.iter().product::<i32>() as u32);
    }
    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input).unwrap(), 62842880);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input).unwrap(), 57600000);
    }
}
