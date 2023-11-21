use fancy_regex::Regex;

pub fn run(input: String) {
    let time = 2503;
    let mut deer = parse_input(input);
    println!("Day14 Pt1: {}", pt1(&deer, time));
    println!("Day14 Pt2: {}", pt2(&mut deer, time));
}

fn parse_input(input: String) -> Vec<Deer> {
    let re = Regex::new(
        r"(.+) can fly (.+) km/s for (.+) seconds, but then must rest for (.+) seconds.",
    )
    .unwrap();
    let mut deer: Vec<Deer> = vec![];
    for line in input.lines() {
        let cap = re.captures(line).unwrap().unwrap();
        deer.push(Deer {
            name: String::from(cap.get(1).unwrap().as_str()),
            speed: cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            dur: cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            rest: cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            points: 0,
            dur_rem: 0,
            rest_rem: 0,
            traveled: 0,
        });
    }
    deer
}

#[derive(Debug)]
#[allow(dead_code)]
struct Deer {
    name: String,
    speed: i32,
    dur: i32,
    rest: i32,
    points: i32,
    dur_rem: i32,
    rest_rem: i32,
    traveled: i32,
}

fn pt1(input: &Vec<Deer>, time: i32) -> i32 {
    let mut max = 0;
    for d in input {
        let cycle = d.dur + d.rest;
        let remaining = time % cycle;
        let distance =
            (((time - remaining) / cycle) * (d.speed * d.dur)) + (d.speed * d.dur.min(remaining));
        max = max.max(distance);
    }
    max
}

fn pt2(input: &mut Vec<Deer>, time: i32) -> i32 {
    let mut i = 0;
    while i < time {
        for d in input.iter_mut() {
            if d.rest_rem == 0 {
                d.dur_rem = d.dur;
                d.rest_rem = d.rest;
            }

            if d.dur_rem > 0 {
                d.dur_rem -= 1;
                d.traveled += d.speed;
            } else {
                d.rest_rem -= 1;
            }
        }
        let max_traveled = input
            .iter()
            .reduce(|d1, d2| if d1.traveled > d2.traveled { d1 } else { d2 })
            .unwrap()
            .traveled;
        for d in input.iter_mut() {
            if d.traveled == max_traveled {
                d.points += 1;
            }
        }

        i += 1;
    }
    input
        .iter()
        .reduce(|d1, d2| if d1.points > d2.points { d1 } else { d2 })
        .unwrap()
        .points
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = String::from(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        );
        let mut deer = parse_input(input);
        assert_eq!(pt2(&mut deer, 1000), 689);
    }
}
