use fancy_regex::Regex;
use std::collections::{HashSet, HashMap};

type Command<'a> = (&'a str, [u32; 2], [u32; 2]);
type LightsPart1 = HashSet<(u32, u32)>;
type LightsPart2 = HashMap<(u32, u32), u32>;

fn parse_command(line: &str) -> Command {
    let line_regex = Regex::new(r"(turn on|turn off|toggle) (\d+,\d+) through (\d+,\d+)").unwrap();
    let parse_number = |num_str: &str| num_str.parse::<u32>().unwrap();

    let captured = line_regex.captures(line).unwrap().unwrap();
    let cmd = captured.get(1).expect("no cmd").as_str();
    let from = match captured.get(2).expect("no from").as_str()
                .split(",").map(parse_number).collect::<Vec<u32>>()[..] {
        [x, y] => [x, y],
        _ => panic!("pattern mismatch")
    };
    let to = match captured.get(3).expect("no to").as_str()
            .split(",").map(parse_number).collect::<Vec<u32>>()[..] {
            [x, y] => [x, y],
            _ => panic!("pattern mismatch")
    };
    (cmd, from, to)
}

fn turn_on(lights: &mut LightsPart1, [from_y, from_x]: [u32; 2], [to_y, to_x]: [u32; 2]) {
    for y in from_y..(to_y+1) {
        for x in from_x..(to_x+1) {
            lights.insert((x, y));
        }
    }
}
fn turn_off(lights: &mut LightsPart1, [from_y, from_x]: [u32; 2], [to_y, to_x]: [u32; 2]) {
    for y in from_y..(to_y+1) {
        for x in from_x..(to_x+1) {
            lights.remove(&(x, y));
        }
    }
}
fn toggle(lights: &mut LightsPart1, [from_y, from_x]: [u32; 2], [to_y, to_x]: [u32; 2]) {
    for y in from_y..(to_y+1) {
        for x in from_x..(to_x+1) {
            if lights.contains(&(x, y)) {
                lights.remove(&(x, y));
            } else {
                lights.insert((x, y));
            }
        }
    }
}

fn increase(
    lights: &mut LightsPart2,
    [from_y, from_x]: [u32; 2],
    [to_y, to_x]: [u32; 2],
    increase_amnt: u32
) {
    for y in from_y..(to_y+1) {
        for x in from_x..(to_x+1) {
             *lights.get_mut(&(x, y)).unwrap() += increase_amnt;
        }
    }
}

fn decrease(lights: &mut LightsPart2, [from_y, from_x]: [u32; 2], [to_y, to_x]: [u32; 2]) {
    for y in from_y..(to_y+1) {
        for x in from_x..(to_x+1) {
            lights.entry((x, y))
                .and_modify(|pos| if *pos > 0 { *pos -= 1 });
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut lights: LightsPart1 = HashSet::new();

    input.trim_end().split("\n").map(parse_command)
        .for_each(|(cmd, from, to)| {
            match cmd {
                "turn on" => turn_on(&mut lights, from, to),
                "turn off" => turn_off(&mut lights, from, to),
                "toggle" => toggle(&mut lights, from, to),
                _ => panic!("unexpected command")
            }
        });
    Some(lights.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lights: LightsPart2 = HashMap::new();
    for y in 0..1000 {
        for x in 0..1000 {
            lights.insert((x, y), 0);
        }
    }
    input.trim_end().split("\n").map(parse_command)
        .for_each(|(cmd, from, to)| {
            match cmd {
                "turn on" => increase(&mut lights, from, to, 1),
                "turn off" => decrease(&mut lights, from, to),
                "toggle" => increase(&mut lights, from, to, 2),
                _ => panic!("unexpected command")
            }
        });
    Some(lights.values().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(998996));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(1001996));
    }
}
