use fancy_regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;

pub fn run(input: String) {
    let mut map = get_happiness(&input);
    println!("Day13 Pt1: {}", pt1(&map));
    println!("Day13 Pt2: {}", pt2(&mut map));
}

fn get_happiness<'a>(input: &'a String) -> HashMap<&'a str, HashMap<&'a str, i32>> {
    let mut map: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let re = Regex::new(r"(.+) would (.+) (\d+) happiness units by sitting next to (.+).").unwrap();
    for l in input.lines() {
        let c = re.captures(l).unwrap().unwrap();
        let a = c.get(1).unwrap().as_str();
        let b = c.get(4).unwrap().as_str();
        let is_gain = c.get(2).unwrap().as_str() == "gain";
        let happiness = if is_gain { 1 } else { -1 } * c.get(3).unwrap().as_str().parse::<i32>().unwrap();

        let val = map.entry(a).or_insert(HashMap::new());
        val.insert(b, happiness);
    }
    map
}

fn pt1(input: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut result:Option<i32> = None;
    for perm in input.keys().permutations(input.len()) {
        let mut i = 0;
        let mut sub_result = 0;
        while i < perm.len() {
            let a = input
            .get(*perm.get(i).unwrap())
            .unwrap();
            let i_b1 = if i == 0 { perm.len() - 1 } else { i - 1 };
            let i_b2 = if i == perm.len() - 1 { 0 } else { i + 1 };
            sub_result += 
                a.get(*perm.get(i_b1).unwrap())
                .unwrap();
            sub_result += 
                a.get(*perm.get(i_b2).unwrap())
                .unwrap();
            i += 1;
        }
        result = match result {
            Some(val) => Some(val.max(sub_result)),
            None => Some(sub_result)
        };
    }
    result.unwrap()
}

fn pt2<'a>(input: &mut HashMap<&'a str, HashMap<&'a str, i32>>) -> i32 {
    add_me(input);
    pt1(&input)
}

fn add_me<'a>(map: &mut HashMap<&'a str, HashMap<&'a str, i32>>) {
    let me = "me";
    for (_, val) in map.iter_mut() {
        val.insert(me, 0);
    }
    let mut my_map = HashMap::new();
    for key in map.keys() {
        my_map.insert(*key, 0);
    }
    map.insert(me, my_map);
}