use fancy_regex::Regex;

pub fn run(input: String) {
    println!("Day5 Pt1: {}", pt1(&input));
    println!("Day5 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let mut i = 0;
    for l in input.lines() {
        if is_nice(l) {
            i += 1;
        }
    }
    i
}

fn is_nice(input: &str) -> bool {
    let mut vowels = 0;
    let mut has_dupe = false;
    let mut last_char: Option<char> = None;
    for c in input.chars() {
        match last_char {
            Some(val) => {
                if val == c {
                    has_dupe = true;
                }
            }
            None => (),
        }

        match last_char {
            Some('a') => {
                if c == 'b' {
                    return false;
                }
            }
            Some('c') => {
                if c == 'd' {
                    return false;
                }
            }
            Some('p') => {
                if c == 'q' {
                    return false;
                }
            }
            Some('x') => {
                if c == 'y' {
                    return false;
                }
            }
            _ => (),
        }

        match c {
            'a' | 'e' | 'i' | 'u' | 'o' => vowels += 1,
            _ => (),
        }

        last_char = Some(c);
    }
    vowels >= 3 && has_dupe
}

fn pt2(input: &String) -> i32 {
    let mut i = 0;
    for l in input.lines() {
        if is_nice_pt2(l) {
            i += 1;
        }
    }
    i
}

fn is_nice_pt2(input: &str) -> bool {
    let re = Regex::new(r"(.)(.).*\1\2").unwrap();
    let re2 = Regex::new(r"(.).\1").unwrap();
    re.is_match(input).unwrap() && re2.is_match(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
        assert_eq!(is_nice_pt2("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_pt2("xxyxx"), true);
        assert_eq!(is_nice_pt2("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_pt2("ieodomkazucvgmuy"), false);
    }
}
