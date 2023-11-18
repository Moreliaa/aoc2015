use fancy_regex::Regex;

pub fn run(input: String) {
    println!("Day12 Pt1: {}", pt1(&input));
    // 24225 too low
    println!("Day12 Pt2: {}", pt1(&input) - pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let all_captures = re.captures_iter(&input);
    let mut sum = 0;
    for cap in all_captures {
        let val = cap.unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
        sum += val;
    }
    sum
}

fn pt2(input: &String) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    let mut is_red = false;
    let mut indent = 0;
    let mut indent_red = 0;
    let mut num: String = String::new();
    while i < input.len() {
        let sub = &input[i..i + 1];
        if i < input.len() - 6 && &input[i..i+6] == ":\"red\"" {
            is_red = true;
            indent_red = indent;
        }
        match sub {
            "-"|"0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => {
                num.push_str(sub);
            },
            "{" => {
                indent+=1;
                if !is_red && num.len() > 0 {
                    sum += num.parse::<i32>().unwrap();
                }
                num = String::new();
            },
            "}" => {
                indent-=1;
                if indent < indent_red {
                    is_red = false;
                }
                if !is_red && num.len() > 0 {
                    sum += num.parse::<i32>().unwrap();
                }
                num = String::new();
            },
            _ => {
                if !is_red && num.len() > 0 {
                    sum += num.parse::<i32>().unwrap();
                }
                num = String::new();

            }
        }
        i += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {

    }
}