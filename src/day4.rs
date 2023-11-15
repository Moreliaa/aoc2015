use md5;
use std::time::Instant;

pub fn run() {
    let input = "iwrupvqb";
    let b1 = Instant::now();
    println!("Day4 Pt1: {}", pt1(input));
    let b2 = Instant::now();
    println!("Time Pt1: {:?}", b2.duration_since(b1));
    println!("Day4 Pt2: {}", pt2(input));
    let b3 = Instant::now();
    println!("Time Pt2: {:?}", b3.duration_since(b2));
}

fn pt1(input: &str) -> i32 {
    let mut i = 1;
    loop {
        let m = md(input, i);
        if &m[0..5] == "00000" {
            break;
        }
        i += 1;
    }
    i
}

fn md(input: &str, i: i32) -> String {
    format!("{:x}", md5::compute(format!("{input}{i}").as_bytes()))
}

fn pt2(input: &str) -> i32 {
    let mut i = 1;
    loop {
        let m = md(input, i);
        if &m[0..6] == "000000" {
            break;
        }
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(pt1("abcdef"), 609043);
        assert_eq!(pt1("pqrstuv"), 1048970);
    }
}
