use fancy_regex::Regex;
use itertools::Itertools;

const CHILDREN: i32 = 3;
const CATS: i32 = 7;
const SAMOYEDS: i32 = 2;
const POMERANIANS: i32 = 3;
const AKITAS: i32 = 0;
const VIZSLAS: i32 = 0;
const GOLDFISH: i32 = 5;
const TREES: i32 = 3;
const CARS: i32 = 2;
const PERFUMES: i32 = 1;

pub fn run(input: String) {
    println!("Day16 Pt1: {}", pt1_2(&input, is_match_pt1));
    println!("Day16 Pt2: {}", pt1_2(&input, is_match_pt2));
}

fn pt1_2<F>(input: &String, matcher: F) -> i32
where
    F: FnOnce(Vec<(&str, i32)>) -> bool + Copy,
{
    let re = Regex::new(r"Sue (\d+): (.+)").unwrap();
    for l in input.lines() {
        let cap = re.captures(&l).unwrap().unwrap();
        let props: Vec<(&str, i32)> = cap
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|prop| {
                let p: (&str, &str) = prop.trim().split(": ").collect_tuple().unwrap();
                let name = p.0;
                let val = p.1.parse::<i32>().unwrap();
                (name, val)
            })
            .collect();
        if matcher(props) {
            return cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
        }
    }
    panic!();
}

fn is_match_pt1(props: Vec<(&str, i32)>) -> bool {
    props.into_iter().fold(true, |a, b| {
        a && match b.0 {
            "children" => CHILDREN == b.1,
            "cats" => CATS == b.1,
            "samoyeds" => SAMOYEDS == b.1,
            "pomeranians" => POMERANIANS == b.1,
            "akitas" => AKITAS == b.1,
            "vizslas" => VIZSLAS == b.1,
            "goldfish" => GOLDFISH == b.1,
            "trees" => TREES == b.1,
            "cars" => CARS == b.1,
            "perfumes" => PERFUMES == b.1,
            _ => panic!(),
        }
    })
}

fn is_match_pt2(props: Vec<(&str, i32)>) -> bool {
    props.into_iter().fold(true, |a, b| {
        a && match b.0 {
            "children" => CHILDREN == b.1,
            "cats" => CATS < b.1,
            "samoyeds" => SAMOYEDS == b.1,
            "pomeranians" => POMERANIANS > b.1,
            "akitas" => AKITAS == b.1,
            "vizslas" => VIZSLAS == b.1,
            "goldfish" => GOLDFISH > b.1,
            "trees" => TREES < b.1,
            "cars" => CARS == b.1,
            "perfumes" => PERFUMES == b.1,
            _ => panic!(),
        }
    })
}
