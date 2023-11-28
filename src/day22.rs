use itertools::Itertools;
use std::collections::HashMap;

pub fn run() {
    let player = Character {
        hp: 50,
        atk_base: 0,
        mana: 500,
        mana_spent: 0,
        effects: vec![],
    };

    let boss = Character {
        hp: 71,
        atk_base: 10,
        mana: 0,
        mana_spent: 0,
        effects: vec![],
    };

    let abilities = ability_map();

    println!(
        "Day22 Pt1: {}",
        overengineer(&player, &boss, &abilities, false)
    );
    println!(
        "Day22 Pt2: {}",
        overengineer(&player, &boss, &abilities, true)
    );
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Character {
    hp: i16,
    atk_base: i16,
    mana: i16,
    mana_spent: i16,
    effects: Vec<Effect>,
}

fn ability_map() -> HashMap<Abilities, Ability> {
    let mut spells: HashMap<Abilities, Ability> = HashMap::new();
    spells.insert(
        Abilities::Attack,
        Ability {
            mana_cost: 0,
            damage: 0,
            self_heal: 0,
            duration: 0,
            ability_type: AbilityType::Attack,
            effect_type: EffectType::NoEffect,
        },
    );
    spells.insert(
        Abilities::MagicMissile,
        Ability {
            mana_cost: 53,
            damage: 4,
            self_heal: 0,
            duration: 0,
            ability_type: AbilityType::Attack,
            effect_type: EffectType::NoEffect,
        },
    );
    spells.insert(
        Abilities::Drain,
        Ability {
            mana_cost: 73,
            damage: 2,
            self_heal: 2,
            duration: 0,
            ability_type: AbilityType::Attack,
            effect_type: EffectType::NoEffect,
        },
    );
    spells.insert(
        Abilities::Shield,
        Ability {
            mana_cost: 113,
            damage: 0,
            self_heal: 0,
            duration: 6,
            ability_type: AbilityType::Buff,
            effect_type: EffectType::Shield,
        },
    );
    spells.insert(
        Abilities::Poison,
        Ability {
            mana_cost: 173,
            damage: 3,
            self_heal: 0,
            duration: 6,
            ability_type: AbilityType::Debuff,
            effect_type: EffectType::Poison,
        },
    );
    spells.insert(
        Abilities::MagicRecharge,
        Ability {
            mana_cost: 229,
            damage: 0,
            self_heal: 0,
            duration: 5,
            ability_type: AbilityType::Buff,
            effect_type: EffectType::MagicRecharge,
        },
    );
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
                }
                EffectType::Poison => {
                    self.hp -= e.damage;
                }
                EffectType::Shield => (),
                _ => (),
            };
        }
        self.effects = self
            .effects
            .into_iter()
            .filter(|e| e.duration > 0)
            .collect_vec();
        self
    }

    fn has_effect(&self, effect_type: EffectType) -> bool {
        self.effects
            .iter()
            .filter(|e| e.effect_type == effect_type && e.is_active)
            .collect::<Vec<_>>()
            .len()
            > 0
    }

    fn add_effect(&mut self, ability: &Ability) {
        self.effects.push(Effect {
            damage: ability.damage,
            duration: ability.duration,
            is_active: false,
            effect_type: ability.effect_type.clone(),
        });
    }

    fn cast(
        &mut self,
        spell: Abilities,
        other_char: &mut Character,
        abilities: &HashMap<Abilities, Ability>,
    ) -> bool {
        let ability = abilities.get(&spell).unwrap();

        if self.mana < ability.mana_cost {
            return false;
        }

        match ability.ability_type {
            AbilityType::Attack => {
                other_char.hp -= dmg(ability.damage + self.atk_base, other_char.get_def());
                self.hp += ability.self_heal;
            }
            AbilityType::Buff => {
                if self.has_effect(ability.effect_type) {
                    return false;
                }
                self.add_effect(&ability);
            }
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

    fn get_def(&self) -> i16 {
        if self.has_effect(EffectType::Shield) {
            7
        } else {
            0
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Effect {
    damage: i16,
    duration: i16,
    is_active: bool,
    effect_type: EffectType,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum EffectType {
    NoEffect,
    Shield,
    Poison,
    MagicRecharge,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Abilities {
    Attack,
    MagicMissile,
    Drain,
    Shield,
    Poison,
    MagicRecharge,
}

struct Ability {
    mana_cost: i16,
    damage: i16,
    self_heal: i16,
    duration: i16,
    ability_type: AbilityType,
    effect_type: EffectType,
}

#[derive(PartialEq, Eq, Hash)]
enum AbilityType {
    Attack,
    Buff,
    Debuff,
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    player: Character,
    boss: Character,
    players_turn: bool,
}

fn overengineer(
    player: &Character,
    boss: &Character,
    abilities: &HashMap<Abilities, Ability>,
    is_hard_mode: bool,
) -> i16 {
    let mut states: Vec<State> = vec![];
    states.push(State {
        player: player.clone(),
        players_turn: true,
        boss: boss.clone(),
    });

    let mut min_mana_spent: Option<i16> = None;
    while states.len() > 0 {
        let s = states.pop().unwrap();
        let result = step(s, min_mana_spent, abilities, is_hard_mode);
        if result.0 {
            min_mana_spent = match min_mana_spent {
                Some(val) => {
                    if result.1[0].player.mana_spent < val {
                        Some(result.1[0].player.mana_spent)
                    } else {
                        Some(val)
                    }
                }
                None => Some(result.1[0].player.mana_spent),
            };
            println!("Success! {}", min_mana_spent.unwrap());
        } else {
            result.1.into_iter().for_each(|r| {
                states.push(r);
            });
        }
    }
    min_mana_spent.unwrap()
}

fn step(
    mut s: State,
    min_mana_spent: Option<i16>,
    abilities: &HashMap<Abilities, Ability>,
    is_hard_mode: bool,
) -> (bool, Vec<State>) {
    let mut next_states: Vec<State> = vec![];

    match min_mana_spent {
        Some(val) if s.player.mana_spent >= val => return (false, vec![]),
        _ => (),
    };

    if s.boss.hp <= 0 {
        return (true, vec![s]);
    }

    if is_hard_mode && s.players_turn {
        s.player.hp -= 1;
    }
    if s.player.hp <= 0 {
        return (false, vec![]);
    }

    s.player = s.player.step_effects();
    s.boss = s.boss.step_effects();

    if s.player.hp <= 0 {
        return (false, vec![]);
    } else if s.boss.hp <= 0 {
        return (true, vec![s]);
    }

    if s.players_turn {
        for spell in [
            Abilities::MagicMissile,
            Abilities::Drain,
            Abilities::Poison,
            Abilities::Shield,
            Abilities::MagicRecharge,
        ]
        .into_iter()
        {
            let mut next_state = s.clone();
            if next_state
                .player
                .cast(spell, &mut next_state.boss, abilities)
            {
                next_state.players_turn = !next_state.players_turn;
                next_states.push(next_state);
            }
        }
    } else {
        let mut next_state = s.clone();
        next_state
            .boss
            .cast(Abilities::Attack, &mut next_state.player, abilities);

        if next_state.player.hp > 0 {
            next_state.players_turn = !next_state.players_turn;
            next_states.push(next_state);
        }
    }
    (false, next_states)
}

fn dmg(atk: i16, def: i16) -> i16 {
    match atk - def {
        val if val <= 0 => 1,
        val => val,
    }
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
            effects: vec![],
        };

        let boss = Character {
            hp: 14,
            atk_base: 8,
            mana: 0,
            mana_spent: 0,
            effects: vec![],
        };

        overengineer(&player, &boss, &ability_map(), false);
    }
}
