use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut current_pos = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::from([current_pos]);
    input.chars().for_each(|ch| {
        match ch {
            '^' => current_pos.0 += 1,
            'v' => current_pos.0 -= 1,
            '<' => current_pos.1 -= 1,
            '>' => current_pos.1 += 1,
            _ => panic!("unexpected input")
        }
        set.insert(current_pos);
    });
    Some(set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut current_pos = [(0, 0), (0, 0)];
    let mut set: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    input.char_indices().for_each(|(idx, ch)| {
        match ch {
            '^' => current_pos[idx%2].0 += 1,
            'v' => current_pos[idx%2].0 -= 1,
            '<' => current_pos[idx%2].1 -= 1,
            '>' => current_pos[idx%2].1 += 1,
            _ => panic!("unexpected input")
        }
        set.insert(current_pos[idx%2]);
    });
    Some(set.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        let lines = input.trim_end().split("\n").collect::<Vec<&str>>();
        assert_eq!(part_one(lines[0]).unwrap(), 2);
        assert_eq!(part_one(lines[1]).unwrap(), 4);
        assert_eq!(part_one(lines[2]).unwrap(), 2);
        assert_eq!(part_one(lines[3]).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        let lines = input.trim_end().split("\n").collect::<Vec<&str>>();
        assert_eq!(part_two(lines[1]).unwrap(), 3);
        assert_eq!(part_two(lines[2]).unwrap(), 11);
        assert_eq!(part_two(lines[3]).unwrap(), 3);
    }
}
