use itertools::Itertools;

pub fn run() {
    let player = Character {
        hp: 100,
        atk_base: 0,
        def_base: 0
    };

    let boss = Character {
        hp: 100,
        atk_base: 8,
        def_base: 2
    };

    let weapons = vec![
        Item {
            name: "Dagger".to_string(),
            cost: 8,
            atk: 4,
            def: 0,
        },
        Item {
            name: "Shortsword".to_string(),
            cost: 10,
            atk: 5,
            def: 0,
        },
        Item {
            name: "Warhammer".to_string(),
            cost: 25,
            atk: 6,
            def: 0,
        },
        Item {
            name: "Longsword".to_string(),
            cost: 40,
            atk: 7,
            def: 0,
        },
        Item {
            name: "Greataxe".to_string(),
            cost: 74,
            atk: 8,
            def: 0,
        },
    ];

    let armor = vec![
        Item {
            name: "Nothing".to_string(),
            cost: 0,
            atk: 0,
            def: 0,
        },
        Item {
            name: "Leather".to_string(),
            cost: 13,
            atk: 0,
            def: 1,
        },
        Item {
            name: "Chainmail".to_string(),
            cost: 31,
            atk: 0,
            def: 2,
        },
        Item {
            name: "Splintmail".to_string(),
            cost: 53,
            atk: 0,
            def: 3,
        },
        Item {
            name: "Bandedmail".to_string(),
            cost: 75,
            atk: 0,
            def: 4,
        },
        Item {
            name: "Platemail".to_string(),
            cost: 102,
            atk: 0,
            def: 5,
        },
    ];

    let rings = vec![
        Item {
            name: "Nothing 1".to_string(),
            cost: 0,
            atk: 0,
            def: 0,
        },
        Item {
            name: "Nothing 2".to_string(),
            cost: 0,
            atk: 0,
            def: 0,
        },
        Item {
            name: "Damage +1".to_string(),
            cost: 25,
            atk: 1,
            def: 0,
        },
        Item {
            name: "Damage +2".to_string(),
            cost: 50,
            atk: 2,
            def: 0,
        },
        Item {
            name: "Damage +3".to_string(),
            cost: 100,
            atk: 3,
            def: 0,
        },
        Item {
            name: "Defense +1".to_string(),
            cost: 20,
            atk: 0,
            def: 1,
        },
        Item {
            name: "Defense +2".to_string(),
            cost: 40,
            atk: 0,
            def: 2,
        },
        Item {
            name: "Defense +3".to_string(),
            cost: 80,
            atk: 0,
            def: 3,
        },
    ];

    println!("Day21 Pt1: {}", pt1(&player, &boss, &weapons, &armor, &rings));
    println!("Day21 Pt2: {}", pt2(&player, &boss, &weapons, &armor, &rings));
}

struct Character {
    hp: i32,
    atk_base: i32,
    def_base: i32
}

struct Item {
    name: String,
    cost: i32,
    atk: i32,
    def: i32,
}

fn pt1(player: &Character, boss: &Character, weapons: &Vec<Item>, armor:&Vec<Item>, rings:&Vec<Item>) -> i32 {
    let mut min_cost: Option<i32> = None;
    for w in weapons {
        for a in armor {
            for r in rings.iter().permutations(2) {
                let cost = w.cost + a.cost + r[0].cost + r[1].cost;
                if min_cost != None && Some(cost) >= min_cost {
                    continue;
                }
                let atk_bonus = w.atk + r[0].atk + r[1].atk;
                let def_bonus = a.def + r[0].def + r[1].def;
                let ttk_player = ttk(&player, atk_bonus, &boss, 0);
                let ttk_boss = ttk(&boss, 0, &player, def_bonus);
                if ttk_player <= ttk_boss {
                    min_cost = Some(cost);
                    println!("{} {} {} {} {} {}", ttk_player, ttk_boss, w.name, a.name, r[0].name, r[1].name);
                }
            }
        }
    }
    min_cost.unwrap()
}

fn ttk(attacker: &Character, atk_bonus: i32, defender: &Character, def_bonus: i32) -> i32 {
    let dmg = match attacker.atk_base + atk_bonus - (defender.def_base + def_bonus) {
        val if val <= 0 => 1,
        val => val
    };
    (defender.hp as f32 / dmg as f32).ceil() as i32
}



fn pt2(player: &Character, boss: &Character, weapons: &Vec<Item>, armor:&Vec<Item>, rings:&Vec<Item>) -> i32 {
    let mut max_cost: Option<i32> = None;
    for w in weapons {
        for a in armor {
            for r in rings.iter().permutations(2) {
                let cost = w.cost + a.cost + r[0].cost + r[1].cost;
                if max_cost != None && Some(cost) <= max_cost {
                    continue;
                }
                let atk_bonus = w.atk + r[0].atk + r[1].atk;
                let def_bonus = a.def + r[0].def + r[1].def;
                let ttk_player = ttk(&player, atk_bonus, &boss, 0);
                let ttk_boss = ttk(&boss, 0, &player, def_bonus);
                if ttk_player > ttk_boss {
                    max_cost = Some(cost);
                    println!("{} {} {} {} {} {}", ttk_player, ttk_boss, w.name, a.name, r[0].name, r[1].name);
                }
            }
        }
    }
    max_cost.unwrap()
}
