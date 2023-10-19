// use advent_of_code::debug;
use std::sync::Mutex;
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn find_divisible_numbers(divisor: u32, part2: bool) -> Vec<u32> {
    let mut res: Vec<u32> = (1..=divisor/2)
        .filter(|&x| {
            divisor % x == 0 &&
            if !part2 {
                true
            } else {
                divisor / x <= 50
            }
        })
        .collect_vec();
    res.push(divisor);
    res
}

pub fn part_one(input: &str) -> Option<u32> {
    let target = input.trim_end().parse::<u32>().unwrap() / 10;

    let found = Mutex::new(target);
    let test_range = if cfg!(test) {
        1..10
    }
    else {
        770_000..780_000
    };
    test_range.into_par_iter().for_each(|num| {
        let tmp = find_divisible_numbers(num, false)
            .iter().sum::<u32>();
        // debug!((num, tmp));
        if tmp >= target {
            let mut found = found.lock().unwrap();
            *found = std::cmp::min(num, *found);
        }
    });
    Some(found.into_inner().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let target = input.trim_end().parse::<u32>().unwrap();

    let found = Mutex::new(target);
    let test_range = if cfg!(test) {
        1..10
    }
    else {
        750_000..800_000
    };
    test_range.into_par_iter().for_each(|num| {
        let tmp = find_divisible_numbers(num, true)
            .iter().map(|n| n*11).sum::<u32>();
        // debug!((num, tmp));
        if tmp >= target {
            let mut found = found.lock().unwrap();
            *found = std::cmp::min(num, *found);
        }
    });
    Some(found.into_inner().unwrap())

}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        let lines = input.trim_end().split("\n").collect_vec();
        assert_eq!(part_one(&lines[0]).unwrap(), 6);
        assert_eq!(part_one(&lines[1]).unwrap(), 6);
        assert_eq!(part_one(&lines[2]).unwrap(), 8);
        assert_eq!(part_one(&lines[3]).unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        let lines = input.trim_end().split("\n").collect_vec();
        assert_eq!(part_two(&lines[0]).unwrap(), 6);
        assert_eq!(part_two(&lines[1]).unwrap(), 6);
        assert_eq!(part_two(&lines[2]).unwrap(), 8);
        assert_eq!(part_two(&lines[3]).unwrap(), 6);
    }
}
