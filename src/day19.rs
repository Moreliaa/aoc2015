use fancy_regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let (mol, replacements) = parse_input(&input);
    println!("Day19 Pt1: {}", pt1(&mol, &replacements));
    println!("Day19 Pt2: {}", pt2(&input));
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
    let mut set:HashSet<String> = HashSet::new();
    for (r, options) in replacements {
        let size = r.len();
        let mut i = 0;
        while i + size <= mol.len() {
            if *r == &mol[i..i + size] {
                for o in options {
                    let new_mol = String::from(&mol[0..i]) + o + &mol[i + size..mol.len()];
                    println!("{new_mol} {o} {r}");
                    set.insert(new_mol);
                }
            }
            i+=1;
        } 
    }
    set.len()
}

fn pt2(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = String::from("H => HO
H => OH
O => HH

HOH");
        let (mol, replacements) = parse_input(&input);
        assert_eq!(pt1(&mol, &replacements), 4);

    }
}