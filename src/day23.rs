use std::collections::HashMap;

pub fn run(input: String) {
    let instr = parse_input(input);
    let reg:HashMap<&str, i128> = HashMap::new();
    println!("Day23 Pt1: {}", pt1_2(&instr, "b", reg));
    let mut reg:HashMap<&str, i128> = HashMap::new();
    reg.insert("a", 1);
    println!("Day23 Pt2: {}", pt1_2(&instr, "b", reg));
}

fn parse_input<'a>(input:String) -> Vec<String> {
    let mut result:Vec<String> = vec![];
    for l in input.lines() {
        result.push(l.to_string());
    }
    result
}

fn pt1_2<'a>(instr: &'a Vec<String>, target: &'a str, mut reg: HashMap<&'a str, i128>) -> i128 {
    let mut pointer: i128 = 0;
    while pointer >= 0 && (pointer as usize) < instr.len() {
        let p = pointer as usize;
        let op = &instr[p][0..3];
        let r = &instr[p][4..];
        match op {
            "hlf" => {
                reg.entry(r).and_modify(|v| *v /= 2).or_insert(0);
                pointer += 1;
            },
            "tpl" => {
                reg.entry(r).and_modify(|v| *v *= 3).or_insert(0);
                pointer += 1;
            },
            "inc" => {
                reg.entry(r).and_modify(|v| *v += 1).or_insert(1);
                pointer += 1;
            },
            "jmp" => {
                let val = r.parse::<i128>().unwrap();
                pointer += val;
            },
            "jie" => {
                let r = &instr[p][4..5];
                if reg.get(r).unwrap_or(&0) % 2 == 0 {
                    let val = instr[p][7..].parse::<i128>().unwrap();
                    pointer += val;
                } else {
                    pointer += 1;
                }
                
            },
            "jio" => {
                let r = &instr[p][4..5];
                if *reg.get(r).unwrap_or(&0) == 1 {
                    let val = instr[p][7..].parse::<i128>().unwrap();
                    pointer += val;
                } else {
                    pointer += 1;
                }
            },
            _ => panic!()
        }
    }
    *reg.get(target).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let input = "inc a
jio a, +2
tpl a
inc a".to_string();
        let instr = parse_input(input);
        println!("Day23 Pt1: {}", pt1_2(&instr, "a", HashMap::new()));
    }
}