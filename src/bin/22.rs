// use advent_of_code::debug;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;


lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"(\d+)").unwrap();

    static ref SKILLS: Skills = Skills {
        missile: Skill { cost: 53, value: 4, turn: 0 },
        drain: Skill { cost: 73, value: 2, turn: 0 },
        shield: Skill { cost: 113, value: 7, turn: 6 },
        poison: Skill { cost: 173, value: 3, turn: 6 },
        charge: Skill { cost: 229, value: 101, turn: 5 },
    };
}

fn parse_input(input: &str) -> Character {
    let boss_status: Vec<i32> = NUM_REGEX
        .captures_iter(input)
        .map(|captured| {
            captured.extract::<1>().0.parse::<i32>().unwrap()
        })
        .collect();
    Character {
        name: "Boss".to_string(),
        current_buff: Buff { poison: 0, shield: 0, charge: 0 },
        hp: boss_status[0], mp: 0,
        atk: boss_status[1], def: 0,
        total_mp_used: 0,
    }
}

struct Skill {
    cost: i32,
    value: i32,
    turn: i32,
}

struct Skills {
    missile: Skill,
    drain: Skill,
    shield: Skill,
    poison: Skill,
    charge: Skill,
}

#[derive(Debug, PartialEq)]
enum State { Player, Boss }

#[derive(Debug, Clone, Copy)]

struct Buff {
    shield: u32,
    poison: u32,
    charge: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Character {
    name: String,
    current_buff: Buff,
    hp: i32, mp: i32,
    atk: i32, def: i32,
    total_mp_used: i32,
}

impl Character { // implement for player only
    fn attack(&mut self, skill: String, boss: &mut Character) {
        match skill.as_str() {
            "missile" => {
                self.mp -= SKILLS.missile.cost;
                self.total_mp_used += SKILLS.missile.cost;
                boss.defense(SKILLS.missile.value);
                // println!("Player casts Magic Missile, dealing {} damage.", SKILLS.missile.value);
            },
            "drain" => {
                self.mp -= SKILLS.drain.cost;
                self.total_mp_used += SKILLS.drain.cost;
                boss.defense(SKILLS.drain.value);
                self.hp += SKILLS.drain.value;
                // println!("Player casts Drain, dealing {} damage, and healing {} hit points.", SKILLS.drain.value, SKILLS.drain.value);
            },
            "shield" => {
                self.mp -= SKILLS.shield.cost;
                self.total_mp_used += SKILLS.shield.cost;
                self.current_buff.shield = SKILLS.shield.turn as u32;
                // println!("Player casts Shield, increasing armor by {}.", SKILLS.shield.value);
            },
            "poison" => {
                self.mp -= SKILLS.poison.cost;
                self.total_mp_used += SKILLS.poison.cost;
                self.current_buff.poison = SKILLS.poison.turn as u32;
                // println!("Player casts Poison.");
            },
            "charge" => {
                self.mp -= SKILLS.charge.cost;
                self.total_mp_used += SKILLS.charge.cost;
                self.current_buff.charge = SKILLS.charge.turn as u32;
                // println!("Player casts Recharge.");
            },
            _ => panic!("unknown skill")
        }
    }

    fn defense(&mut self, receive_damage: i32) {
        let damage: i32 = receive_damage - self.def;
        // println!("{} defense, received damage {}.", self.name, damage);
        self.hp -= if damage > 0 { damage } else { 1 };
    }
}


fn solve(player: Character, boss: Character, state: State, hardmode: bool) -> i32 {
    // println!("\n-- {:?} turn --", &state);
    // println!("- Player has {} hp, {} turn armor buff, {} mp", player.hp, player.current_buff.shield, player.mp);
    // println!("- Boss has {} hp", boss.hp);

    let mut new_player = player.clone();
    let mut new_boss = boss.clone();

    if hardmode && state == State::Player { new_player.hp -= 1; }

    if new_player.current_buff.shield > 0 {
        new_player.current_buff.shield -= 1;
        new_player.def = SKILLS.shield.value;
    } else {
        new_player.def = 0;
    }
    if new_player.current_buff.poison > 0 {
        new_player.current_buff.poison -= 1;
        new_boss.hp -= SKILLS.poison.value;
        // println!("Poison deals {} damage; its timer is now {}.", SKILLS.poison.value, new_player.current_buff.poison);
    }
    if new_player.current_buff.charge > 0 {
        new_player.current_buff.charge -= 1;
        new_player.mp += SKILLS.charge.value;
        // println!("Recharge provides {} mana; its timer is now {}.", SKILLS.charge.value, new_player.current_buff.charge);
    }

    if new_boss.hp <= 0 {
        // println!("This kills the boss, and the player wins.");
        return new_player.total_mp_used;
    }
    if new_player.hp <= 0 {
        // println!("Player died, boss wins");
        return i32::MAX;
    }

    match state {
        State::Player => {
            let available_skills: Vec<String> = vec![
                ("missile", SKILLS.missile.cost),
                ("drain", SKILLS.drain.cost),
                ("shield", SKILLS.shield.cost),
                ("poison", SKILLS.poison.cost),
                ("charge", SKILLS.charge.cost),
            ].iter()
            .filter(|(skill, mp_cost)| {
                new_player.mp >= *mp_cost &&
                match *skill {
                    "shield" => new_player.current_buff.shield == 0,
                    "poison" => new_player.current_buff.poison == 0,
                    "charge" => new_player.current_buff.charge == 0,
                    _ => true
                }
            })
            .map(|(str, _)| str.to_string())
            .collect_vec();

            // if no skill to cast (insufficient mp) and no buff = return
            if available_skills.len() == 0 {
                if [
                    new_player.current_buff.shield,
                    new_player.current_buff.poison,
                    new_player.current_buff.charge,
                ].iter().all(|v| *v == 0) { return i32::MAX; }
                else {
                    // if buff still active, then proceed without doing nothing
                    return solve(new_player, new_boss, State::Boss, hardmode);
                }
            }
            available_skills.iter().map(|skill| {
                let mut next_player = new_player.clone();
                let mut next_boss = new_boss.clone();
                next_player.attack(skill.to_string(), &mut next_boss);
                if next_boss.hp < 1 {
                    // println!("This kills the boss, and the player wins.");
                    return next_player.total_mp_used;
                }
                solve(next_player, next_boss, State::Boss, hardmode)
            }).min().unwrap_or_else(|| i32::MAX)
        },
        State::Boss => {
            new_player.defense(boss.atk);
            if new_player.hp < 1 {
                // println!("Player died, boss wins");
                return i32::MAX;
            }
            solve(new_player, new_boss, State::Player, hardmode)
        },
    }
}

fn get_characters(input: &str) -> (Character, Character) {
    let boss = parse_input(input);
    let player_status = if cfg!(test) { [10, 250] } else { [50, 500] };

    let player = Character {
        name: "Player".to_string(),
        current_buff: Buff { poison: 0, shield: 0, charge: 0 },
        hp: player_status[0],
        mp: player_status[1],
        atk: 0, def: 0,
        total_mp_used: 0,
    };
    (player, boss)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (default_user, boss) = get_characters(input);
    let res = solve(default_user, boss, State::Player, false);
    // debug!(res);
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (default_user, boss) = get_characters(input);
    let res = solve(default_user, boss, State::Player, true);
    // debug!(res);
    Some(res as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one("Hit Points: 13\nDamage: 8\n"), Some(226));
        assert_eq!(part_one("Hit Points: 14\nDamage: 8\n"), Some(641));
    }

    #[test]
    fn test_part_two() {
        // let input = advent_of_code::read_file("examples", 22);
        // assert_eq!(part_two("Hit Points: 13\nDamage: 8\n"), Some(i32::MAX as u32));
        // assert_eq!(part_two("Hit Points: 14\nDamage: 8\n"), Some(i32::MAX as u32));
    }
}
