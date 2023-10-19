// use advent_of_code::debug;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp;

struct Item {
    name: String,
    cost: u32,
    atk: u32,
    def: u32,
}

lazy_static! {
    static ref WEAPONS: Vec<Item> = vec![
        Item { name: "Dagger".to_string(),      cost:  8, atk: 4, def: 0 },
        Item { name: "Shortsword".to_string(),  cost: 10, atk: 5, def: 0 },
        Item { name: "Warhammer".to_string(),   cost: 25, atk: 6, def: 0 },
        Item { name: "Longsword".to_string(),   cost: 40, atk: 7, def: 0 },
        Item { name: "Greataxe".to_string(),    cost: 74, atk: 8, def: 0 },
    ];
    static ref ARMORS: Vec<Item> = vec![
        Item { name: "None".to_string(),        cost: 0,   atk: 0, def: 0 },
        Item { name: "Leather".to_string(),     cost: 13,  atk: 0, def: 1 },
        Item { name: "Chainmail".to_string(),   cost: 31,  atk: 0, def: 2 },
        Item { name: "Splintmail".to_string(),  cost: 53,  atk: 0, def: 3 },
        Item { name: "Bandedmail".to_string(),  cost: 75,  atk: 0, def: 4 },
        Item { name: "Platemail".to_string(),   cost: 102, atk: 0, def: 5 },
    ];
    static ref RINGS: Vec<Item> = vec![
        Item { name: "None".to_string(),        cost: 0,   atk: 0, def: 0 },
        Item { name: "Damage +1".to_string(),   cost: 25,  atk: 1, def: 0 },
        Item { name: "Damage +2".to_string(),   cost: 50,  atk: 2, def: 0 },
        Item { name: "Damage +3".to_string(),   cost: 100, atk: 3, def: 0 },
        Item { name: "Defense +1".to_string(),  cost: 20,  atk: 0, def: 1 },
        Item { name: "Defense +2".to_string(),  cost: 40,  atk: 0, def: 2 },
        Item { name: "Defense +3".to_string(),  cost: 80,  atk: 0, def: 3 },
    ];
    static ref PLAYERS: Vec<Player> = {
        let mut players: Vec<Player> = vec![];
        for weapon in WEAPONS.iter() {
            for armor in ARMORS.iter() {
                for r1 in RINGS.iter() {
                    for r2 in RINGS.iter() {
                        // no duplicate item except None
                        if r1.name != "None" && r1.name == r2.name { continue; }
                        let atk_sum = weapon.atk + r1.atk + r2.atk;
                        let def_sum = armor.def + r1.def + r2.def;
                        let cost_sum = weapon.cost + armor.cost + r1.cost + r2.cost;
                        players.push(
                            Player {
                                name: "player".to_string(),
                                hp: if cfg!(test) { 12 } else { 100 },
                                atk: atk_sum, def: def_sum, cost: cost_sum
                            }
                        )
                    }
                }
            }
        }
        players
    };
}

fn parse_input(input: &str) -> Player {
    let nums = input.trim_end().split("\n").map(|line| {
        line.split(" ").last().unwrap().parse::<u32>().unwrap()
    }).collect_vec();
    Player { name: "boss".to_string(), hp: nums[0] as i32, atk: nums[1], def: nums[2], cost: 0 }
}

enum State {
    Player,
    Boss,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Player {
    name: String,
    hp: i32,
    atk: u32,
    def: u32,
    cost: u32,
}

impl Player {
    fn defense(&mut self, opponent: &Player) {
        let dmg = opponent.atk - self.def;
        // by rule, if dmg is negative then at least deal dmg 1
        self.hp -= if dmg > 0 { dmg } else { 1 } as i32;
        if self.hp < 0 { self.hp = 0; }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let boss_tmpl = parse_input(input);
    let mut state = State::Player;

    let mut min_cost = u32::MAX;
    for player_tmpl in PLAYERS.iter() {
        let mut player = player_tmpl.clone();
        let mut boss = boss_tmpl.clone();

        // if player cost more than prev, no need to exec
        if player.cost >= min_cost { continue; }

        while player.hp > 0 && boss.hp > 0 {
            match state {
                State::Player => {
                    boss.defense(&player);
                    state = State::Boss;
                },
                State::Boss => {
                    player.defense(&boss);
                    state = State::Player;
                }
            }
        }
        if player.hp > 0 { // if player win
            // debug!(&player_tmpl);
            // debug!(player);
            // debug!(boss);
            // println!("----------------------------------");
            min_cost = cmp::min(min_cost, player.cost);
        }
    }
    Some(min_cost)

}

pub fn part_two(input: &str) -> Option<u32> {
    let boss_tmpl = parse_input(input);
    let mut state = State::Player;

    let mut most_cost = u32::MIN;
    for player_tmpl in PLAYERS.iter() {
        let mut player = player_tmpl.clone();
        let mut boss = boss_tmpl.clone();

        // if player cost less than prev, no need to exec
        if player.cost <= most_cost { continue; }

        while player.hp > 0 && boss.hp > 0 {
            match state {
                State::Player => {
                    boss.defense(&player);
                    state = State::Boss;
                },
                State::Boss => {
                    player.defense(&boss);
                    state = State::Player;
                }
            }
        }
        if player.hp <= 0 { // if player lose
            // debug!(&player_tmpl);
            // debug!(player);
            // debug!(boss);
            // println!("----------------------------------");
            most_cost = cmp::max(most_cost, player.cost);
        }
    }
    Some(most_cost)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_part_one() {
    //     let input = advent_of_code::read_file("examples", 21);
    //     assert_eq!(part_one(&input), None);
    // }

    // #[test]
    // fn test_part_two() {
    //     let input = advent_of_code::read_file("examples", 21);
    //     assert_eq!(part_two(&input), None);
    // }
}
