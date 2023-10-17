use regex::Regex;
use serde_json::{Map, Value, from_str};

pub fn part_one(input: &str) -> Option<i32> {
    let input = input.trim_end();
    let num_regex = Regex::new(r"-?\d+").unwrap();
    let res = num_regex.find_iter(input).fold(0, |acc, found| {
        acc + found.as_str().parse::<i32>().unwrap()
    });
    Some(res)
}

fn remove_red(json: &Value, re: &Regex) -> Value {
    match json {
        Value::Object(obj) => {
            let mut new_obj = Map::new();
            for (key, value) in obj {
                match value {
                    Value::String(str) => {
                        if !re.is_match(str.as_str()) {
                            new_obj.insert(key.clone(), value.clone());
                        } else {
                            return Value::Null;
                        }
                    },
                    other => {
                        new_obj.insert(key.clone(), remove_red(&other, re));
                    },
                };
            }
            Value::Object(new_obj)
        }
        Value::Array(arr) => {
            let mut new_arr = Vec::new();
            for item in arr {
                new_arr.push(remove_red(item, re));
            }
            Value::Array(new_arr)
        }
        other => other.clone(),
    }
}

pub fn part_two(input: &str) -> Option<i32> {
    let red_regex = Regex::new(r"red").unwrap();
    let json: Value = from_str(input.trim_end()).unwrap();
    let filtered_json = remove_red(&json, &red_regex);

    let num_regex = Regex::new(r"-?\d+").unwrap();
    let res = num_regex.find_iter(&filtered_json.to_string()).fold(0, |acc, found| {
        acc + found.as_str().parse::<i32>().unwrap()
    });
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        let lines: Vec<&str> = input.trim_end().split("\n").collect();
        assert_eq!(part_one(&lines[0]).unwrap(), 6);
        assert_eq!(part_one(&lines[1]).unwrap(), 6);
        assert_eq!(part_one(&lines[2]).unwrap(), 3);
        assert_eq!(part_one(&lines[3]).unwrap(), 3);
        assert_eq!(part_one(&lines[4]).unwrap(), 0);
        assert_eq!(part_one(&lines[5]).unwrap(), 0);
        assert_eq!(part_one(&lines[6]).unwrap(), 0);
        assert_eq!(part_one(&lines[7]).unwrap(), 0);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        let lines: Vec<&str> = input.trim_end().split("\n").collect();
        assert_eq!(part_two(&lines[0]).unwrap(), 6);
        assert_eq!(part_two(&lines[1]).unwrap(), 6);
        assert_eq!(part_two(&lines[2]).unwrap(), 3);
        assert_eq!(part_two(&lines[3]).unwrap(), 3);
        assert_eq!(part_two(&lines[4]).unwrap(), 0);
        assert_eq!(part_two(&lines[5]).unwrap(), 0);
        assert_eq!(part_two(&lines[6]).unwrap(), 0);
        assert_eq!(part_two(&lines[7]).unwrap(), 0);
        assert_eq!(part_two(&lines[8]).unwrap(), 4);
        assert_eq!(part_two(&lines[9]).unwrap(), 0);
        assert_eq!(part_two(&lines[10]).unwrap(), 6);
    }
}
