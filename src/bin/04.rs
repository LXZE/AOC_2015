use md5;
use rayon::prelude::*;
use std::sync::Mutex;

fn solve(str: &str, pattern: &str, start: u32, limit: u32) -> Option<u32> {
    (start..limit)
        .into_par_iter()
        .find_first(|&res| {
            let str = format!("{}{}", str, res.to_string());
            let hash = format!("{:x}", md5::compute(str));
            hash.starts_with(pattern)
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input.trim(), "00000", 0, 2_000_000).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let num_threads = 4;
    let chunk_size = 2_500_000;
    let result = Mutex::new(None);

    (0..num_threads).into_par_iter().for_each(|i| {
        let start = i * chunk_size;
        let end = start + chunk_size;
        let partial_result = solve(input.trim(), "000000", start, end);

        if let Some(partial) = partial_result {
            let mut res = result.lock().unwrap();
            if res.is_none() {
                *res = Some(partial);
            }
        }
    });
    result.into_inner().unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        let lines = input.trim_end().split("\n").collect::<Vec<&str>>();
        assert_eq!(part_one(lines[0]).unwrap(), 609043);
        assert_eq!(part_one(lines[1]).unwrap(), 1048970);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
