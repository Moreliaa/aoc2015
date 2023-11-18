use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let input = String::from("hepxcrrq").chars().collect_vec();
    let result_pt1 = pt1_pt2(input.clone());
    println!("Day11 Pt1: {}", result_pt1);
    println!("Day11 Pt2: {}", pt1_pt2(result_pt1.chars().collect_vec()));
}

fn pt1_pt2(mut input: Vec<char>) -> String {
    input = input.into_iter().rev().collect_vec();

    loop {
        input = increment(input);
        if is_valid(&input) {
            break;
        }
    }

    input.into_iter().rev().collect::<String>()
}

fn is_valid(chars: &Vec<char>) -> bool {
    let mut pairs: HashSet<char> = HashSet::new();
    let not_allowed = ['i', 'o', 'l'];
    let mut has_straight = false;
    let mut last_u: u32 = 0;
    let mut straight_count = 0;
    for c in chars {
        if not_allowed.contains(c) {
            return false;
        }
        let u = u32::from(*c);
        match u {
            val if val == last_u => {
                pairs.insert(*c);
                straight_count = 0;
            }
            val if last_u != 0 && val == last_u - 1 => {
                straight_count += 1;
                if straight_count == 2 {
                    has_straight = true;
                }
            }
            _ => {
                straight_count = 0;
            }
        };
        last_u = u;
    }
    has_straight && pairs.len() > 1
}

fn increment(chars: Vec<char>) -> Vec<char> {
    let min = u32::from('a');
    let max = u32::from('z');
    let (i, o, l) = (u32::from('i'), u32::from('o'), u32::from('l'));

    let mut carry = 1;
    let mut result: Vec<char> = vec![];
    for c in chars {
        let mut u = u32::from(c);

        if carry > 0 {
            carry -= 1;
            u = match u {
                val if val == max => {
                    carry += 1;
                    min
                }
                val if val == i - 1 || val == o - 1 || val == l - 1 => u + 2,
                _ => u + 1,
            };
        }
        result.push(char::from_u32(u).unwrap());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(vec!['d', 'a'], increment(vec!['c', 'a']));
        assert_eq!(vec!['a', 'd'], increment(vec!['z', 'c']));
        test_valid("hijklmmn", false);
        test_valid("abbceffg", false);
        test_valid("abbcegjk", false);
        test_valid("ghjaabcc", true);
        let test_pt1 = String::from("ghijklmn").chars().collect_vec();
        assert_eq!("ghjaabcc", pt1_pt2(test_pt1));
    }

    fn test_valid(input: &str, expected: bool) {
        let test = String::from(input).chars().rev().collect_vec();
        assert_eq!(expected, is_valid(&test), "{input}");
    }
}
