pub fn part_one(input: &str) -> Option<i32> {
    let mut res: i32 = 0;
    for ch in input.chars() {
        res += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut res: i32 = 0;
    for (idx, ch) in input.chars().enumerate() {
        res += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        if res == -1 {
            res = idx as i32 + 1;
            break;
        }
    }
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input).unwrap(), -3);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input).unwrap(), 1);
    }
}
