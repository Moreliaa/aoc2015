pub fn run(input: String) {
    println!("Day8 Pt1: {}", pt1(&input));
    println!("Day8 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let mut count_chars = 0;
    let mut count_chars_in_mem = 0;
    for l in input.lines() {
        let c = char_of_code(l);
        let m = num_chars_mem(l);
        count_chars += c;
        count_chars_in_mem += m;
    }
    count_chars - count_chars_in_mem
}

fn char_of_code(input: &str) -> i32 {
    input.len() as i32
}

fn num_chars_mem(input: &str) -> i32 {
    let mut i = 0;
    let mut count = -2;
    while i < input.len() {
        if &input[i..i + 1] == "\\" {
            match &input[i + 1.. i + 2] {
                "x" => {
                    i += 3;
                },
                _ => {
                    i += 1;
                }
            }
        }
        i += 1;
        count += 1;
    }
    count
}

fn pt2(input: &String) -> i32 {
    let mut count_chars = 0;
    let mut count_chars_in_mem = 0;
    for l in input.lines() {
        let l = escape(l);
        let l = l.as_str();
        let c = char_of_code(l);
        let m = num_chars_mem(l);
        count_chars += c;
        count_chars_in_mem += m;
    }
    count_chars - count_chars_in_mem
}

fn escape<'a>(input: &'a str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut i = 1;

    output.push_str("\"\\\"");
    while i < input.len() - 1 {
        let c = &input[i..i + 1];
        if c == "\\" {
            output.push_str("\\\\");
            match &input[i + 1.. i + 2] {
                "x" => {
                    output.push_str(&input[i + 1..=i + 3]);
                    i += 3;
                },
                "\"" => {
                    i += 1;
                    output.push_str("\\\"");
                },
                "\\" => {
                    i += 1;
                    output.push_str("\\\\");
                },
                _ => panic!("unexpected!")
            }
        } else {
            output.push_str(c);
        }
        i += 1;
    }
    output.push_str("\\\"\"");
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test() {
        let file = fs::read_to_string("./test/day8test.txt").unwrap();
        let mut lines = file.lines();
        let next = lines.next().unwrap();
        assert_eq!(char_of_code(next), 2);
        assert_eq!(num_chars_mem(next), 0);
        println!("{}", escape(next));
        let next = lines.next().unwrap();
        assert_eq!(char_of_code(next), 5);
        assert_eq!(num_chars_mem(next), 3);
        println!("{}", escape(next));
        let next = lines.next().unwrap();
        assert_eq!(char_of_code(next), 10);
        assert_eq!(num_chars_mem(next), 7);
        println!("{}", escape(next));
        let next = lines.next().unwrap();
        assert_eq!(char_of_code(next), 6);
        assert_eq!(num_chars_mem(next), 1);
        println!("{}", escape(next));
        let next = lines.next().unwrap();
        assert_eq!(char_of_code(next), 6);
        assert_eq!(num_chars_mem(next), 3);
        println!("{}", escape(next));
    }
}