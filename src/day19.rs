use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let (mol, replacements) = parse_input(&input);
    println!("Day19 Pt1: {}", pt1(&mol, &replacements));
    println!("Day19 Pt2: {}", pt2(&mol));
}

fn parse_input<'a>(input: &'a String) -> (String, HashMap<&'a str, Vec<&'a str>>) {
    let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
    let re = Regex::new(r"(.+) => (.+)").unwrap();
    let mut is_finished = false;
    for line in input.lines() {
        if is_finished {
            return (String::from(line), replacements);
        }
        let cap = match re.captures(line).unwrap() {
            Some(result) => result,
            _ => {
                is_finished = true;
                continue;
            }
        };
        let first = cap.get(1).unwrap().as_str();
        let second = cap.get(2).unwrap().as_str();
        replacements
            .entry(first)
            .and_modify(|e| e.push(second))
            .or_insert(vec![second]);
    }
    panic!();
}

fn pt1(mol: &String, replacements: &HashMap<&str, Vec<&str>>) -> usize {
    let mut set: HashSet<String> = HashSet::new();
    set.insert(mol.clone());
    (set, _) = step(set, &String::from("A"), &replacements);
    set.len()
}

fn step(
    set: HashSet<String>,
    target: &String,
    replacements: &HashMap<&str, Vec<&str>>,
) -> (HashSet<String>, bool) {
    let mut next_map: HashSet<String> = HashSet::new();
    for key in set {
        for (r, options) in replacements {
            let size = r.len();
            let mut i = 0;
            while i + size <= key.len() {
                if *r == &key[i..i + size] {
                    for o in options {
                        let new_mol = String::from(&key[0..i]) + o + &key[i + size..key.len()];
                        if new_mol == *target {
                            return (next_map, true);
                        }
                        next_map.insert(new_mol);
                    }
                }
                i += 1;
            }
        }
    }
    (next_map, false)
}

fn pt2(mol: &String) -> i32 {
    let mut i = 0;

    let mut elem_count = 0;
    let mut rn_ar_count = 0;
    let mut y_count = 0;
    let chars: Vec<char> = mol.chars().collect();
    while i < chars.len() {
        let c = chars.get(i).unwrap();
        if char::is_uppercase(*c) {
            elem_count += 1;
        }
        if *c == 'Y' {
            y_count += 1;
        }
        match chars.get(i + 1) {
            Some(val) => {
                if (*c == 'A' && *val == 'r') || (*c == 'R' && *val == 'n') {
                    rn_ar_count += 1;
                }
            }
            None => (),
        };

        i += 1;
    }
    return elem_count - rn_ar_count - 2 * y_count - 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = String::from(
            "H => HO
H => OH
O => HH

HOH",
        );
        let (mol, replacements) = parse_input(&input);
        assert_eq!(pt1(&mol, &replacements), 4);
    }
}
