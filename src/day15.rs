use fancy_regex::Regex;

pub fn run(input: String) {
    let ingr = parse_input(input);
    println!("Day15 Pt1: {}", pt1(&ingr));
    println!("Day15 Pt2: {}", pt2(&ingr));
}

fn parse_input(input: String) -> Vec<Ingr> {
    let re =
        Regex::new(r".+: capacity (.+), durability (.+), flavor (.+), texture (.+), calories (.+)")
            .unwrap();
    let mut result = vec![];
    for l in input.lines() {
        let cap = re.captures(l).unwrap().unwrap();
        result.push(Ingr {
            cap: cap.get(1).unwrap().as_str().parse::<i128>().unwrap(),
            dur: cap.get(2).unwrap().as_str().parse::<i128>().unwrap(),
            fla: cap.get(3).unwrap().as_str().parse::<i128>().unwrap(),
            tex: cap.get(4).unwrap().as_str().parse::<i128>().unwrap(),
            cal: cap.get(5).unwrap().as_str().parse::<i128>().unwrap(),
        });
    }
    result
}

struct Ingr {
    cap: i128,
    dur: i128,
    fla: i128,
    tex: i128,
    cal: i128,
}

fn pt1(input: &Vec<Ingr>) -> i128 {
    let size = input.len();
    let mut counters: Vec<i128> = vec![0; size];
    let counter_max = 100;
    let mut max_score = 0;
    while !counters.iter().fold(true, |a, b| a && *b == counter_max) {
        let mut i = 0;
        loop {
            if counters[i] < counter_max {
                counters[i] += 1;
                break;
            } else {
                counters[i] = 0;
                i += 1;
            }
        }
        if counters.iter().sum::<i128>() != 100 {
            continue;
        }
        max_score = max_score.max(score_perm(&input, &counters).0);
    }
    max_score
}

fn score_perm(input: &Vec<Ingr>, perm: &Vec<i128>) -> (i128, i128) {
    let (mut t_cap, mut t_dur, mut t_fla, mut t_tex, mut t_cal) = (0, 0, 0, 0, 0);
    let mut i = 0;
    while i < perm.len() {
        t_cap += input[i].cap * perm[i];
        t_dur += input[i].dur * perm[i];
        t_fla += input[i].fla * perm[i];
        t_tex += input[i].tex * perm[i];
        t_cal += input[i].cal * perm[i];
        i += 1;
    }
    (
        0.max(t_cap) * 0.max(t_dur) * 0.max(t_fla) * 0.max(t_tex),
        t_cal,
    )
}

fn pt2(input: &Vec<Ingr>) -> i128 {
    let size = input.len();
    let mut counters: Vec<i128> = vec![0; size];
    let counter_max = 100;
    let mut max_score = 0;
    while !counters.iter().fold(true, |a, b| a && *b == counter_max) {
        let mut i = 0;
        loop {
            if counters[i] < counter_max {
                counters[i] += 1;
                break;
            } else {
                counters[i] = 0;
                i += 1;
            }
        }
        if counters.iter().sum::<i128>() != 100 {
            continue;
        }
        let result = score_perm(&input, &counters);
        if result.1 != 500 {
            continue;
        }
        max_score = max_score.max(result.0);
    }
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = String::from(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        );
        let ingr = parse_input(input);
        let perm = vec![44, 56];
        assert_eq!(score_perm(&ingr, &perm).0, 62842880);
        assert_eq!(pt1(&ingr), 62842880);
    }
}
