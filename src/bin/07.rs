use std::collections::{HashMap, HashSet};
use fancy_regex::Regex;

#[derive(Debug, Clone, PartialEq)]
struct Expr<'a> {
    ops: &'a str,
    src_l: &'a str,
    src_r: &'a str,
    target: &'a str,
}

fn parse_code(line: &str) -> Expr {
    let stmt_regex = Regex::new(
        r"^(?<expr>.+)\s->\s(?<tgt>\D{1,2})$"
    ).unwrap();
    let stmt_captured = stmt_regex.captures(line).unwrap().unwrap();
    let expr = stmt_captured.name("expr").unwrap().as_str();
    let target = stmt_captured.name("tgt").unwrap().as_str();

    let expr_regex = Regex::new(
        r"(?<l>(\D{1,2}|\d+)\s)?(?<ops>[A-Z]+\s)?(?<r>(\D{1,2}|\d+))"
    ).unwrap();
    let expr_captured = expr_regex.captures(expr).unwrap().unwrap();
    match (expr_captured.name("l"), expr_captured.name("ops"), expr_captured.name("r")) {
        (Some(src_l), Some(ops), Some(src_r)) => Expr {
            src_l: src_l.as_str().trim(), ops: ops.as_str().trim(), src_r: src_r.as_str(), target
        },
        (None, Some(ops), Some(src_r)) => Expr {
            src_l: "", ops: ops.as_str().trim(), src_r: src_r.as_str(), target
        },
        (None, None, Some(src_r)) => Expr {
            src_l: "", ops: "ASSIGN", src_r: src_r.as_str(), target
        },
        _ => panic!("unexpected expression"),
    }
}

fn try_parse_or_get<'a>(text: &str, mem: &HashMap<&str, u16>) -> u16 {
    if text.chars().any(|c| c.is_numeric()) {
        return text.parse::<u16>().unwrap() as u16
    } else {
        *mem.get(text).unwrap()
    }
}

fn compute(mem: &HashMap<&str, u16>, expr: &Expr) -> Result<u16, &'static str> {
    match expr.ops {
        "ASSIGN" => {
            Ok(try_parse_or_get(expr.src_r, mem))
        },
        "NOT" => {
            Ok(!try_parse_or_get(expr.src_r, mem))
        },
        "AND" => {
            Ok(
                try_parse_or_get(expr.src_l, mem)
                & try_parse_or_get(expr.src_r, mem)
            )
        },
        "OR" => {
            Ok(
                try_parse_or_get(expr.src_l, mem)
                | try_parse_or_get(expr.src_r, mem)
            )
        },
        "LSHIFT" => {
            Ok(
                try_parse_or_get(expr.src_l, mem)
                << try_parse_or_get(expr.src_r, mem)
            )
        },
        "RSHIFT" => {
            Ok(
                try_parse_or_get(expr.src_l, mem)
                >> try_parse_or_get(expr.src_r, mem)
            )
        },
        err => {
            println!("{:?}", err);
            panic!("unexpected operation")
        },
    }
}

fn get_code_from_target<'a>(codes: &Vec<Expr<'a>>, target: &str) -> Expr<'a> {
    match codes.iter().find(|&expr| expr.target == target) {
        Some(val) => val.clone(),
        None => {
            // println!("get {} error", target);
            // println!("{:?}", &codes);
            panic!("reference undefined")
        },
    }
}

fn dfs<'a>(current: &'a str, visited: &mut HashSet<&'a str>, stack: &mut Vec<Expr<'a>>, codes: &Vec<Expr<'a>>) -> bool {
    if visited.contains(current) { return true }
    visited.insert(current);
    let required_expr = get_code_from_target(codes, current);
    for source in vec![required_expr.src_l, required_expr.src_r]
        .iter().filter(|&&a| {
            !a.chars().any(|c| c.is_numeric()) && a != ""
        }).map(|&e| e).collect::<Vec<&str>>() {
            if visited.contains(source) { continue }
            if !dfs(source, visited, stack, codes) { return false }
        }
    stack.push(required_expr);
    true
}

fn sort<'a>(codes: &Vec<Expr<'a>>, start: &'a str) -> Vec<Expr<'a>> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut stack: Vec<Expr<'a>> = vec![];
    dfs(start, &mut visited, &mut stack, codes);
    stack
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mem: HashMap<&str, u16> = HashMap::new();
    let mut codes: Vec<Expr> = input.trim_end().split("\n").map(parse_code).collect();
    codes = sort(&codes, "a");
    for expr in codes {
        // println!("{:?}", &expr);
        match compute(&mem, &expr) {
            Ok(val) => { mem.insert(expr.target,  val); },
            Err(_) => { panic!("wrong order code") },
        };
    }
    // println!("{:?}", &mem);
    let res = mem.get("a");
    match res {
        Some(res) => Some(*res as u32),
        None => None,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let signal_a = part_one(input).unwrap().to_string();
    
    let mut mem: HashMap<&str, u16> = HashMap::new();
    let mut codes: Vec<Expr> = input.trim_end().split("\n").map(parse_code).collect();
    codes.retain(|expr| expr.target != "b");
    codes.push(Expr { ops: "ASSIGN", src_l: "", src_r: &signal_a, target: "b" });
    codes = sort(&codes, "a");

    for expr in codes {
        // println!("{:?}", &expr);
        match compute(&mem, &expr) {
            Ok(val) => { mem.insert(expr.target,  val); },
            Err(_) => { panic!("wrong order code") },
        };
    }
    // println!("{:?}", &mem);
    let res = mem.get("a");
    match res {
        Some(res) => Some(*res as u32),
        None => None,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(parse_code("123 -> x"), Expr { ops: "ASSIGN", src_l: "", src_r: "123", target: "x" });
        assert_eq!(parse_code("y -> x"), Expr { ops: "ASSIGN", src_l: "", src_r: "y", target: "x" });
        assert_eq!(parse_code("y -> xy"), Expr { ops: "ASSIGN", src_l: "", src_r: "y", target: "xy" });
        assert_eq!(parse_code("yy -> xy"), Expr { ops: "ASSIGN", src_l: "", src_r: "yy", target: "xy" });
        
        assert_eq!(parse_code("a AND b -> c"), Expr { ops: "AND", src_l: "a", src_r: "b", target: "c" });
        assert_eq!(parse_code("1 AND b -> c"), Expr { ops: "AND", src_l: "1", src_r: "b", target: "c" });
        assert_eq!(parse_code("a AND 1 -> c"), Expr { ops: "AND", src_l: "a", src_r: "1", target: "c" });
        assert_eq!(parse_code("aa AND bb -> cc"), Expr { ops: "AND", src_l: "aa", src_r: "bb", target: "cc" });
        
        assert_eq!(parse_code("NOT 1 -> x"), Expr { ops: "NOT", src_l: "", src_r: "1", target: "x" });
        assert_eq!(parse_code("NOT a -> x"), Expr { ops: "NOT", src_l: "", src_r: "a", target: "x" });
        assert_eq!(parse_code("NOT aa -> x"), Expr { ops: "NOT", src_l: "", src_r: "aa", target: "x" });
        
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input).unwrap(), 123);
    }

    #[test]
    fn test_part_two() {
        // let input = advent_of_code::read_file("examples", 7);
        // assert_eq!(part_two(&input), None);
    }
}
