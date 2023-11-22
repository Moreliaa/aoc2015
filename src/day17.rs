use itertools::Itertools;
use std::collections::HashMap;

pub fn run(input: String) {
    let input = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect_vec();
    let result = pt1_2(&input, 150);
    println!("Day17 Pt1: {}", result.0);
    println!("Day17 Pt2: {}", result.1);
}

fn pt1_2(input: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut map: HashMap<usize, i32> = HashMap::new();
    let pt1_result = perm(&input[..], 0, &mut map, 0, target).unwrap();

    let min_key = map.keys().reduce(|a, b| a.min(b)).unwrap();
    (pt1_result, *map.get(min_key).unwrap())
}

fn perm(
    counters: &[i32],
    containers_used: usize,
    mut map: &mut HashMap<usize, i32>,
    current: i32,
    target: i32,
) -> Option<i32> {
    if current == target {
        map.entry(containers_used)
            .and_modify(|val| *val += 1)
            .or_insert(1);
        return Some(1);
    }
    if counters.len() == 0 || current > target {
        return None;
    }
    let incr = counters[0];
    let mut result = 0;
    match perm(&counters[1..], containers_used, &mut map, current, target) {
        Some(val) => result += val,
        None => (),
    };

    match perm(
        &counters[1..],
        containers_used + 1,
        &mut map,
        current + incr,
        target,
    ) {
        Some(val) => result += val,
        None => (),
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = vec![20, 15, 10, 5, 5];
        let result = pt1_2(&input, 25);
        assert_eq!(result.0, 4);
        assert_eq!(result.1, 3);
    }
}
