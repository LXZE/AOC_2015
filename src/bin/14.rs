// use advent_of_code::debug;
use std::{cmp, collections::HashSet};
use regex::Regex;

fn parse_line(line: &str) -> Deer {
    let line_regex = Regex::new(
        r#"(?<name>\w+) can fly (?<speed>\d+) km/s for (?<time_sprint>\d+) seconds, but then must rest for (?<time_rest>\d+) seconds."#
    ).unwrap();
    let captured = line_regex.captures(line).unwrap();
    Deer::new(
        captured.name("name").unwrap().as_str().to_string(),
        captured.name("speed").unwrap().as_str().parse::<u32>().unwrap(),
        captured.name("time_sprint").unwrap().as_str().parse::<u32>().unwrap(),
        captured.name("time_rest").unwrap().as_str().parse::<u32>().unwrap(),
    )
}

#[derive(Debug)]
struct Deer {
    name: String,
    status: u8, // 1 for sprint, 0 for rest
    speed: u32,
    time_sprint: u32,
    time_rest: u32,
    phase_time_left: u32,
    travelled_km: u32,
    point: u32,
}

impl Deer {
    fn new(name: String, speed: u32, time_sprint: u32, time_rest: u32) -> Deer {
        Deer {
            name, speed, time_sprint, time_rest,
            status: 1, phase_time_left: time_sprint, travelled_km: 0, point: 0
        }
    }

    fn get_distance(&self) -> u32 { self.travelled_km }
    fn get_point(&self) -> u32 { self.point }
    fn increase_point(&mut self) -> () { self.point += 1; }

    fn exec(&mut self) -> () {
        match self.status {
            1 => { // if sprinting
                if self.phase_time_left > 0 {
                    self.travelled_km += self.speed;
                    self.phase_time_left -= 1;
                    if self.phase_time_left == 0 {
                        self.phase_time_left = self.time_rest;
                        self.status = 0;
                    }
                } else {
                    self.phase_time_left = self.time_rest;
                }
            },
            0 => { // if resting
                if self.phase_time_left > 0 {
                    self.phase_time_left -= 1;
                    if self.phase_time_left == 0 {
                        self.phase_time_left = self.time_sprint;
                        self.status = 1;
                    }
                } else {
                    self.phase_time_left = self.time_sprint;
                }
            },
            _ => panic!("unexpected status"),
        }
    }
}

fn find_leaders(deers: &Vec<Deer>) -> Vec<String> {
    let mut max: u32 = 0;
    deers.iter().for_each(|deer| max = cmp::max(max, deer.get_distance()));
    deers.iter()
        .filter(|&deer| deer.get_distance() == max)
        .map(|deer| deer.name.clone())
        .collect::<Vec<String>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut deers: Vec<Deer> = input.trim_end().split("\n").map(parse_line).collect();
    let time = 2503;
    for _ in 0..time {
        deers.iter_mut().for_each(|deer: &mut Deer| deer.exec());
    }
    let mut max = 0;
    deers.iter().for_each(|deer| max = cmp::max(max, deer.get_distance()));
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut deers: Vec<Deer> = input.trim_end().split("\n").map(parse_line).collect();
    let time = 2503;
    for _ in 0..time {
        deers.iter_mut().for_each(|deer: &mut Deer| deer.exec());
        let leaders: HashSet<String> = HashSet::from_iter(find_leaders(&deers).into_iter());
        deers.iter_mut().for_each(|deer: &mut Deer| {
            if leaders.contains(&deer.name) { deer.increase_point(); }
        });
    }
    let mut max = 0;
    deers.iter().for_each(|deer| max = cmp::max(max, deer.get_point()));
    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input).unwrap(), 2660);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input).unwrap(), 1564);
    }
}
