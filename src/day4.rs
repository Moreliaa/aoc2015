use md5;

pub fn run() {
    let input = "iwrupvqb".to_string();
    println!("Pt1: {}", pt1(&input));
    println!("Pt2: {}", pt2(&input));
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
    let input = input.to_string() + i.to_string().as_str();
    format!("{:x}", md5::compute(input.as_bytes()))
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