fn parse_vec(input: &str) -> Vec<Vec<u32>> {
    input.trim_end()
        .split("\n").collect::<Vec<&str>>()
        .into_iter().map(|s|
            s.split("x").collect::<Vec<&str>>()
            .into_iter().map(|val|
                val.parse().unwrap()
            ).collect()
        ).collect::<Vec<Vec<u32>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = parse_vec(input)
        .iter().map(|line|
            match line[..] {
                [l,w,h] => {
                    let side = [l*w, w*h, h*l];
                    let min_val = *side.iter().min().unwrap();
                    (side.iter().sum::<u32>() * 2) + min_val
                },
                _ => panic!("parse Error"),
            }
        ).collect::<Vec<u32>>()
        .iter().sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = parse_vec(input)
        .iter_mut().map(|line| {
            line.sort();
            match line[..] {
                [min, mid, _max] => {
                    let wrap = (min+mid)*2;
                    let bow = line.iter().product::<u32>();
                    wrap + bow
                },
                _ => panic!("parse Error"),
            }
        }
        ).collect::<Vec<u32>>()
        .iter().sum::<u32>();
    Some(res)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input).unwrap(), 101);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input).unwrap(), 48);
    }
}
