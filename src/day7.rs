use fancy_regex::Regex;
use std::collections::HashMap;

pub fn run(input: String) {
    let result_pt1 = pt1(&input);
    println!("Day7 Pt1: {}", result_pt1);
    println!("Day7 Pt2: {}", pt2(&input, result_pt1));
}

fn pt1(input: &String) -> u16 {
    *build_circuit(input, HashMap::new()).get("a").unwrap()
}

fn build_circuit<'a>(
    input: &'a String,
    mut result: HashMap<&'a str, u16>,
) -> HashMap<&'a str, u16> {
    let re = Regex::new(r"(.+) -> (.+)").unwrap();
    let re_pair = Regex::new(r"(.+) (.+) (.+)").unwrap();
    let re_not = Regex::new(r"NOT (.+)").unwrap();
    let mut commands = input.lines().collect::<Vec<&str>>();
    let mut commands_remaining: Vec<&str> = vec![];

    while commands.len() > 0 {
        for l in commands {
            let c = re.captures(l).unwrap().unwrap();
            let cmd = c.get(1).unwrap().as_str();
            let wire = c.get(2).unwrap().as_str();
            if (result.contains_key(wire)) {
                continue;
            }

            if re_pair.is_match(cmd).unwrap() {
                let c = re_pair.captures(cmd).unwrap().unwrap();
                let a = c.get(1).unwrap().as_str();
                let instr = c.get(2).unwrap().as_str();
                let b = c.get(3).unwrap().as_str();

                let val_a = match a.parse::<u16>() {
                    Ok(val) => val,
                    _ => match result.get(a) {
                        Some(val) => *val,
                        None => {
                            commands_remaining.push(l);
                            continue;
                        }
                    },
                };

                let val_b = match b.parse::<u16>() {
                    Ok(val) => val,
                    _ => match result.get(b) {
                        Some(val) => *val,
                        None => {
                            commands_remaining.push(l);
                            continue;
                        }
                    },
                };

                let val_wire = match instr {
                    "AND" => val_a & val_b,
                    "OR" => val_a | val_b,
                    "LSHIFT" => val_a << val_b,
                    "RSHIFT" => val_a >> val_b,
                    _ => panic!("unexpected!"),
                };
                result.insert(wire, val_wire);
            } else if re_not.is_match(cmd).unwrap() {
                let c = re_not.captures(cmd).unwrap().unwrap();
                let a = c.get(1).unwrap().as_str();
                let val_a = match result.get(a) {
                    Some(val) => *val,
                    None => {
                        commands_remaining.push(l);
                        continue;
                    }
                };
                result.insert(wire, !val_a);
            } else {
                let val_cmd = match cmd.parse::<u16>() {
                    Ok(val) => val,
                    _ => match result.get(cmd) {
                        Some(val) => *val,
                        None => {
                            commands_remaining.push(l);
                            continue;
                        }
                    },
                };
                result.insert(wire, val_cmd);
            }
        }
        commands = commands_remaining;
        commands_remaining = vec![];
    }
    result
}

fn get_val(result: &HashMap<&str, u16>, input: &str) -> Option<u16> {
    Some(0)
}

fn pt2(input: &String, pt1_result: u16) -> u16 {
    let mut result = HashMap::new();
    result.insert("b", pt1_result);
    *build_circuit(input, result).get("a").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = String::from(
            "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        );
        let map = build_circuit(&input, HashMap::new());
        assert_eq!(*map.get("d").unwrap(), 72);
        assert_eq!(*map.get("e").unwrap(), 507);
        assert_eq!(*map.get("f").unwrap(), 492);
        assert_eq!(*map.get("g").unwrap(), 114);
        assert_eq!(*map.get("h").unwrap(), 65412);
        assert_eq!(*map.get("i").unwrap(), 65079);
        assert_eq!(*map.get("x").unwrap(), 123);
        assert_eq!(*map.get("y").unwrap(), 456);
    }
}
