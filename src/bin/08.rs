use regex::Regex;

fn calc_len(str: &str) -> u32 {
    let chars_len = str.len();
    let regex = Regex::new(r"(\\\W)|(\\x\w{2})|(\w)").unwrap();
    let captured = regex.find_iter(&str[1..chars_len-1]);
    (chars_len - captured.enumerate().count()) as u32
}

fn calc_len_encode(str: &str) -> u32 {
    let chars_len = str.len();
    let expanded_len = str.chars().map(|char| {
        if char == '\\' || char == '"' { 2 } else { 1 }
    }).sum::<u32>() + 2; // +2 for surround quote
    expanded_len - chars_len as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim_end().split("\n").map(calc_len).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.trim_end().split("\n").map(calc_len_encode).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(19));
    }
}
