use fancy_regex::Regex;
use itertools::Itertools;
use std::collections::HashMap;

pub fn run(input: String) {
    let distances = get_distances(&input);
    println!("Day9 Pt1: {}", pt1_2(&distances, pt1_match));
    println!("Day9 Pt2: {}", pt1_2(&distances, pt2_match));
}

fn get_distances<'a>(input: &'a String) -> HashMap<&'a str, HashMap<&'a str, i32>> {
    let mut distances: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let re = Regex::new(r"(.+) to (.+) = (.+)").unwrap();
    for l in input.lines() {
        let c = re.captures(l).unwrap().unwrap();
        let t1 = c.get(1).unwrap().as_str();
        let t2 = c.get(2).unwrap().as_str();
        let dist = c.get(3).unwrap().as_str().parse::<i32>().unwrap();

        let val = distances.entry(t1).or_insert(HashMap::new());
        val.insert(t2, dist);
        let val = distances.entry(t2).or_insert(HashMap::new());
        val.insert(t1, dist);
    }
    distances
}

fn pt1_2<F>(distances: &HashMap<&str, HashMap<&str, i32>>, matcher: F) -> i32
where
    F: FnOnce(Option<i32>, i32) -> Option<i32> + Copy,
{
    let mut result = None;
    for perm in distances.keys().permutations(distances.len()) {
        let mut i = 0;
        let mut sub_result = 0;
        while i < perm.len() - 1 {
            sub_result += distances
                .get(*perm.get(i).unwrap())
                .unwrap()
                .get(*perm.get(i + 1).unwrap())
                .unwrap();
            i += 1;
        }
        result = matcher(result, sub_result);
    }
    result.unwrap()
}

fn pt1_match(result: Option<i32>, sub_result: i32) -> Option<i32> {
    match result {
        None => Some(sub_result),
        Some(val) => Some(val.min(sub_result)),
    }
}

fn pt2_match(result: Option<i32>, sub_result: i32) -> Option<i32> {
    match result {
        None => Some(sub_result),
        Some(val) => Some(val.max(sub_result)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = String::from(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        );
        assert_eq!(pt1_2(&get_distances(&input), pt1_match), 605);
    }
}
