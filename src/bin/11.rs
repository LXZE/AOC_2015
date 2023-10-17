use itertools::Itertools;

fn char_to_u32(char: char) -> u32 {
    char as u32 - 97
}
fn vecu32_to_string(v: Vec<u32>) -> String {
    v.into_iter().map(|c| char::from_u32(c + 97).unwrap()).join("")
}

fn rule_1_valid(v: &Vec<u32>) -> bool {
    for i in 0..v.len()-2 {
        match v[i..i+3] {
            [a,b,c] => {
                if a+1 == b && a+2 == c {
                    return true
                }
            },
            _ => panic!("len mismatch"),
        }
    }
    false
}

fn rule_2_valid(v: &Vec<u32>) -> bool {
    let invalid_char = ['i', 'l', 'o'].into_iter().map(char_to_u32).collect::<Vec<u32>>();
    !v.iter().any(|c| invalid_char.contains(&c))
}

fn rule_3_valid(v: &Vec<u32>) -> bool {
    let mut found_once = false;
    let mut last_index = -1;
    for i in 0..v.len()-1 {
        if v[i] == v[i+1] {
            if !found_once {
                found_once = true;
                last_index = i as i32;
            }
            else if last_index + 1 == i as i32 {
                last_index = -1;
            } else {
                return true
            }
        }
    }
    false
}

fn is_valid(str: &Vec<u32>) -> bool {
    // println!("{:?}", vecu32_to_string(str.clone()));
    // println!("{} {} {}", rule_1_valid(str), rule_2_valid(str), rule_3_valid(str));
    rule_1_valid(str) && rule_2_valid(str) && rule_3_valid(str)
}

fn increase(v: &mut Vec<u32>) {
    for i in (0..v.len()).rev() {
        v[i] += 1;
        if v[i] > 25 {
            v[i] = 0;
        } else {
            return;
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut result = input.trim_end().to_string().chars().map(char_to_u32).collect::<Vec<u32>>();
    loop {
        if is_valid(&result) { break; } else { increase(&mut result); }
    }
    Some(vecu32_to_string(result))
}

pub fn part_two(input: &str) -> Option<String> {
    let input = part_one(input).unwrap();
    let mut result = input.chars().map(char_to_u32).collect::<Vec<u32>>();
    increase(&mut result);
    loop {
        if is_valid(&result) { break; } else { increase(&mut result); }
    }
    Some(vecu32_to_string(result))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let v = "abcdffaa".to_string().chars().map(char_to_u32).collect::<Vec<u32>>();
        // is_valid(&v);

        let input = advent_of_code::read_file("examples", 11);
        let lines = input.trim().split("\n").collect::<Vec<&str>>();
        assert_eq!(part_one(&lines[0]).unwrap(), "abcdffaa");
        assert_eq!(part_one(&lines[1]).unwrap(), "ghjaabcc");
    }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 11);
    //     assert_eq!(part_two(&input), None);
    // }
}
