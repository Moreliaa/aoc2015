use aoc_lib::map2d::Map2D;
use fancy_regex::Regex;

pub fn run(input: String) {
    println!("Day6 Pt1: {}", pt1(&input));
    println!("Day6 Pt2: {}", pt2(&input));
}

fn read_line(input: &str) -> (&str, i32, i32, i32, i32) {
    let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let cap = re.captures(input).unwrap().unwrap();
    (
        cap.get(1).unwrap().as_str(),
        cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        cap.get(5).unwrap().as_str().parse::<i32>().unwrap(),
    )
}

fn pt1(input: &String) -> i32 {
    let mut map = Map2D::<char>::new(1000, 1000, '.');
    for l in input.lines() {
        let instr = read_line(l);
        let coords = (instr.1, instr.2, instr.3, instr.4);
        match instr.0 {
            "turn on" => turn_on(&mut map, coords),
            "turn off" => turn_off(&mut map, coords),
            "toggle" => toggle(&mut map, coords),
            _ => panic!("unexpected: {l}"),
        }
    }
    map.aggregate(|val| if *val == '#' { 1 } else { 0 })
}

fn turn_on(map: &mut Map2D<char>, coords: (i32, i32, i32, i32)) {
    for x in coords.0..=coords.2 {
        for y in coords.1..=coords.3 {
            map.set(x, y, '#');
        }
    }
}

fn turn_off(map: &mut Map2D<char>, coords: (i32, i32, i32, i32)) {
    for x in coords.0..=coords.2 {
        for y in coords.1..=coords.3 {
            map.set(x, y, '.');
        }
    }
}

fn toggle(map: &mut Map2D<char>, coords: (i32, i32, i32, i32)) {
    for x in coords.0..=coords.2 {
        for y in coords.1..=coords.3 {
            let c = match map.get(x, y).unwrap() {
                '.' => '#',
                _ => '.',
            };
            map.set(x, y, c);
        }
    }
}

fn pt2(input: &String) -> i32 {
    let mut map = Map2D::<i32>::new(1000, 1000, 0);
    for l in input.lines() {
        let instr = read_line(l);
        let coords = (instr.1, instr.2, instr.3, instr.4);
        match instr.0 {
            "turn on" => change(&mut map, coords, 1),
            "turn off" => change(&mut map, coords, -1),
            "toggle" => change(&mut map, coords, 2),
            _ => panic!("unexpected: {l}"),
        }
    }
    map.aggregate(|val| *val)
}

fn change(map: &mut Map2D<i32>, coords: (i32, i32, i32, i32), val: i32) {
    for x in coords.0..=coords.2 {
        for y in coords.1..=coords.3 {
            map.set(x, y, (*map.get(x, y).unwrap() + val).max(0));
        }
    }
}
