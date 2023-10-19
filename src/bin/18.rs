use itertools::Itertools;
use lazy_static::lazy_static;
// use advent_of_code::debug;

type Map = Vec<Vec<bool>>;

lazy_static! {
    static ref ADJACENT_PATTERN: [(i8, i8); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1)
    ];
}

fn parse_map(input: &str) -> Map {
    let mut result = vec![];
    input.trim_end().split("\n").for_each(|line| {
        result.push(line.chars().map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("unexpected chars"),
        }).collect_vec());
    });
    result
}

#[allow(dead_code)]
fn print_map(map: &Map) {
    for row in map {
        println!("{}", row.iter().map(|state| match state {
            true => '#',
            false => '.',
        }).join(""));
    }
}

fn generate_adjacent(idx_row: usize, idx_col: usize, limit: usize) -> Vec<(usize, usize)> {
    let limit_range: std::ops::Range<i8> = 0..limit as i8;
    ADJACENT_PATTERN.clone().iter()
        .map(|(d_row, d_col)| (idx_row as i8 + d_row, idx_col as i8 + d_col))
        .filter(|(r, c)| limit_range.contains(r) && limit_range.contains(c))
        .map(|(r, c)| (usize::from(r as u8), usize::from(c as u8)))
        .collect_vec()
}
 
fn update_map(map: &Map, part2: bool) -> Map {
    let mut new_map = map.clone();
    let limit = map.len();
    for (idx_row, row) in map.iter().enumerate() {
        for (idx_col, current_state) in row.iter().enumerate() {
            if part2
            && (idx_row == 0 || idx_row == limit - 1)
            && (idx_col == 0 || idx_col == limit - 1) {
                continue;
            }
            let neighbors = generate_adjacent(idx_row, idx_col, limit).iter()
                .map(|(r, c)| map[*r][*c] ).collect_vec();
            match current_state {
                true => {
                    if !(2..=3).contains(&neighbors.iter().filter(|n| **n).count())  {
                        new_map[idx_row][idx_col] = false;
                    }
                },
                false => {
                    if *&neighbors.iter().filter(|n| **n).count() == 3  {
                        new_map[idx_row][idx_col] = true;
                    }
                }
            }
        }
    }
    new_map
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse_map(input);
    let step = if cfg!(test) { 4 } else { 100 }; 
    (0..step).for_each(|_| {
        map = update_map(&map, false);
    });
    Some(map.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|state| **state).count() as u32
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse_map(input);
    let b = map.len() - 1;
    map[0][0] = true;
    map[0][b] = true;
    map[b][b] = true;
    map[b][0] = true;
    // print_map(&map);
    // println!("-------------------------");

    let step = if cfg!(test) { 5 } else { 100 }; 
    (0..step).for_each(|_| {
        map = update_map(&map, true);
        // print_map(&map);
        // println!("-------------------------");
    });
    Some(map.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|state| **state).count() as u32
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // debug!(generate_adjacent(0, 0, 6));
        // debug!(generate_adjacent(0, 1, 6));
        // debug!(generate_adjacent(1, 0, 6));
        // debug!(generate_adjacent(1, 1, 6));

        // debug!(generate_adjacent(4, 4, 6));
        // debug!(generate_adjacent(4, 5, 6));
        // debug!(generate_adjacent(5, 4, 6));
        // debug!(generate_adjacent(5, 5, 6));

        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input).unwrap(), 4);
    }

    #[test]
    fn test_part_two() {
        let input = r###"
##.#.#
...##.
#....#
..#...
#.#..#
####.#
        "###;
        // let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input.trim()).unwrap(), 17);
    }
}
