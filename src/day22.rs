use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let player = Character {
        hp: 50,
        atk_base: 0,
        mana: 500,
        mana_spent: 0,
        effects: vec![]
    };

    let boss = Character {
        hp: 71,
        atk_base: 10,
        mana: 0,
        mana_spent: 0,
        effects: vec![]
    };

    println!("Day22 Pt1: {}", pt1(&player, &boss));
    println!("Day22 Pt2: {}", pt2(&player, &boss));
}

#[derive(Clone)]
struct Character {
    hp: i32,
    atk_base: i32,
    mana: i32,
    mana_spent: i32,
    effects: Vec<Effect>
}

fn ability_map() -> HashMap<Abilities, Ability> {
    let mut spells:HashMap<Abilities, Ability> = HashMap::new();
    spells.insert(Abilities::Attack, Ability {
        name: "Attack".to_string(),
        mana_cost: 0,
        damage: 0,
        self_heal: 0,
        duration: 0,
        ability_type: AbilityType::Attack,
        effect_type: EffectType::NoEffect
    });
    spells.insert(Abilities::MagicMissile, Ability {
        name: "Magic Missile".to_string(),
        mana_cost: 53,
        damage: 4,
        self_heal: 0,
        duration: 0,
        ability_type: AbilityType::Attack,
        effect_type: EffectType::NoEffect
    });
    spells.insert(Abilities::Drain, Ability {
        name: "Drain".to_string(),
        mana_cost: 73,
        damage: 2,
        self_heal: 2,
        duration: 0,
        ability_type: AbilityType::Attack,
        effect_type: EffectType::NoEffect
    });
    spells.insert(Abilities::Shield, Ability {
        name: "Shield".to_string(),
        mana_cost: 113,
        damage: 0,
        self_heal: 0,
        duration: 6,
        ability_type: AbilityType::Buff,
        effect_type: EffectType::Shield
    });
    spells.insert(Abilities::Poison, Ability {
        name: "Poison".to_string(),
        mana_cost: 173,
        damage: 3,
        self_heal: 0,
        duration: 6,
        ability_type: AbilityType::Debuff,
        effect_type: EffectType::Poison
    });
    spells.insert(Abilities::MagicRecharge, Ability {
        name: "Recharge".to_string(),
        mana_cost: 229,
        damage: 0,
        self_heal: 0,
        duration: 5,
        ability_type: AbilityType::Buff,
        effect_type: EffectType::MagicRecharge
    });
    spells
}

impl Character {
    fn step_effects(mut self) -> Character {
        for e in self.effects.iter_mut() {
            e.duration -= 1;
            e.is_active = true;
            match e.effect_type {
                EffectType::MagicRecharge => {
                    self.mana += 101;
                },
                EffectType::Poison => {
                    self.hp -= e.damage;
                },
                EffectType::Shield => (),
                _ => ()
            };
        }
        self.effects = self.effects.into_iter().filter(|e| e.duration > 0).collect_vec();
        self
    }

    fn has_effect(&self, effect_type: EffectType) -> bool {
        self.effects.iter().filter(|e| e.effect_type == effect_type && e.is_active).collect::<Vec<_>>().len() > 0
    }

    fn add_effect(&mut self, ability:&Ability) {
        self.effects.push(Effect {
            damage: ability.damage,
            duration: ability.duration,
            is_active: false,
            effect_type: ability.effect_type.clone()
        });
    } 

    fn cast(&mut self, spell:Abilities, other_char: &mut Character) -> bool {
        let ability_map = ability_map();
        let ability = ability_map.get(&spell).unwrap();
        
        if self.mana < ability.mana_cost {
            return false;
        }

        match ability.ability_type {
            AbilityType::Attack => {
                other_char.hp -= dmg(ability.damage + self.atk_base, other_char.get_def());
                self.hp += ability.self_heal;
            },
            AbilityType::Buff => {
                if self.has_effect(ability.effect_type) {
                    return false;
                }
                self.add_effect(&ability);
            },
            AbilityType::Debuff => {
                if other_char.has_effect(ability.effect_type) {
                    return false;
                }
                other_char.add_effect(&ability);
            }
        };

        self.mana -= ability.mana_cost;
        self.mana_spent += ability.mana_cost;
        true
    }

    fn get_def(&self) -> i32 {
        if self.has_effect(EffectType::Shield) { 7 } else { 0 }
    }
}

#[derive(Clone)]
#[derive(Copy)]
struct Effect {
    damage: i32,
    duration: i32,
    is_active: bool,
    effect_type: EffectType,
}

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum EffectType { NoEffect, Shield, Poison, MagicRecharge }
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Debug)]
enum Abilities { Attack, MagicMissile, Drain, Shield, Poison, MagicRecharge }
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
enum AbilityType { Attack, Buff, Debuff }
struct Ability {
    name: String,
    mana_cost: i32,
    damage: i32,
    self_heal: i32,
    duration: i32,
    ability_type: AbilityType,
    effect_type: EffectType
}

#[derive(Clone)]
struct State {
    player: Character,
    boss: Character,
    players_turn: bool,
    log: Vec<String>
}

fn pt1(player: &Character, boss: &Character) -> i32 {
    let mut states: Vec<State> = vec![];
    states.push(State {
        player: player.clone(),
        players_turn: true,
        boss:boss.clone(),
        log: vec![]
    });
    let mut min_mana_spent:Option<i32> = None;
    let mut result_log: Option<Vec<String>> = None;
    while states.len() > 0 {
        let mut next_states: Vec<State> = vec![];
        for s in states.into_iter() {
            let mut result = step(s, min_mana_spent);
            if result.0 {
                min_mana_spent = match min_mana_spent {
                    Some(val) => {
                        if result.1[0].player.mana_spent < val {
                            result_log = Some(result.1[0].log.clone());
                            Some(result.1[0].player.mana_spent)
                        } else {
                            Some(val)
                        }
                    },
                    None => {
                        result_log = Some(result.1[0].log.clone());
                        Some(result.1[0].player.mana_spent)
                    }
                };
            } else {
                next_states.append(&mut result.1);
            }
        }
        states = next_states;
    }

    for l in result_log.unwrap() {
        println!("{l}");
    }
    min_mana_spent.unwrap()
}

fn step(mut s: State, min_mana_spent: Option<i32>) -> (bool, Vec<State>) {
    let mut next_states: Vec<State> = vec![];

    match min_mana_spent {
        Some(val) if s.player.mana_spent >= val => return (false, vec![]),
        _ => ()
    };

    if s.boss.hp <= 0 {
        return (true, vec![s]);
    } else if s.player.hp <= 0 {
        return (false, vec![]);
    }

    s.player = s.player.step_effects();
    s.boss = s.boss.step_effects();

    if s.player.hp <= 0 {
        return (false, vec![]);
    } else if s.boss.hp <= 0 {
        return (true, vec![s])
    }

    let names = ["MM", "D", "P", "S", "MR"];

    for (idx, spell) in [Abilities::MagicMissile, Abilities::Drain,Abilities::Poison,Abilities::Shield,Abilities::MagicRecharge].into_iter().enumerate() {
        let mut next_state = s.clone();
        if next_state.players_turn {
            if next_state.player.cast(spell, &mut next_state.boss) {
            
                next_state.log.push(format!("Player cast {:?}. Player: {} HP {} MP, Boss: {} HP", names[idx], next_state.player.hp, next_state.player.mana, next_state.boss.hp));
                next_state.players_turn = !next_state.players_turn;
                next_states.push(next_state);
            }
        } else {
            next_state.boss.cast(Abilities::Attack, &mut next_state.player);

            next_state.log.push(format!("Boss attacks. Player: {} HP {} MP, Boss: {} HP", next_state.player.hp, next_state.player.mana, next_state.boss.hp));
            next_state.players_turn = !next_state.players_turn;
            next_states.push(next_state);
        }
        
    }

    (false, next_states)
}

fn dmg(atk: i32, def: i32) -> i32 {
    match atk - def {
        val if val <= 0 => 1,
        val => val
    }
}



fn pt2(player: &Character, boss: &Character) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let player = Character {
            hp: 10,
            atk_base: 0,
            mana: 250,
            mana_spent: 0,
            effects: vec![]
        };
    
        let boss = Character {
            hp: 14,
            atk_base: 8,
            mana: 0,
            mana_spent: 0,
            effects: vec![]
        };

        pt1(&player, &boss);
    }
}
