fn expand(str: String) -> String {
    let mut res: Vec<(char, u32)> = vec![];
    for char in str.chars() {
        if res.last().is_none() || res.last().unwrap().0 != char {
            res.push((char, 1));
        } else {
            res.len();
            res.last_mut().unwrap().1 += 1;
        }
    }

    res.into_iter().fold("".to_string(), |acc, (char, count)| {
        acc + &count.to_string() + &char.to_string()
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = input.trim_end().to_string();
    for _ in 0..40 {
        res = expand(res);
    }
    Some(res.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res = input.trim_end().to_string();
    for _ in 0..50 {
        res = expand(res);
    }
    Some(res.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input).unwrap(), 82350);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input).unwrap(), 1166642);
    }
}
