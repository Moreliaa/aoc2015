use fancy_regex::Regex;
use aoc_lib::tree;

pub fn run(input: String) {
    println!("Day12 Pt1: {}", pt1(&input));
    println!("Day12 Pt2: {}", pt2(&input));
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

struct Node {
    sum: i32,
    is_red: bool
}

fn pt2(input: &String) -> i32 {
    let mut tree = tree::Tree::new(Node {
        sum: 0,
        is_red: false
    });
    let mut current_node = tree.get_root();
    let mut i = 0;
    let mut is_red = false;
    let mut indent = 0;
    let mut indent_red = -1;
    let mut num: String = String::new();
    while i < input.len() {
        let sub = &input[i..i + 1];
        // TODO toss out all the values on same ident level before the red
        if i < input.len() - 6 && &input[i..i+6] == ":\"red\"" {
            current_node.borrow_mut().val.is_red = true;
            indent_red = indent;
        }
        match sub {
            "-"|"0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => {
                num.push_str(sub);
            },
            "{" => {
                indent+=1;
                if !is_red && num.len() > 0 {
                    current_node.borrow_mut().val.sum += num.parse::<i32>().unwrap();
                }
                num = String::new();
                let parent_is_red = current_node.borrow().val.is_red;
                current_node = tree.add_child(&current_node, Node {
                    sum: 0,
                    is_red: parent_is_red
                });
            },
            "}" => {
                indent-=1;
                if !is_red && num.len() > 0 {
                    current_node.borrow_mut().val.sum += num.parse::<i32>().unwrap();
                }
                if indent < indent_red {
                    is_red = false;
                    indent_red = -1;
                }
                num = String::new();
                let next_node = current_node.borrow().get_parent().unwrap();
                current_node = next_node;
            },
            _ => {
                if !is_red && num.len() > 0 {
                    current_node.borrow_mut().val.sum += num.parse::<i32>().unwrap();
                }
                num = String::new();

            }
        }
        i += 1;
    }
    tree.aggregate_root(|node| if node.borrow().val.is_red { 0 } else { node.borrow().val.sum })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(pt2(&String::from("[1,2,3]")), 6);
        assert_eq!(pt2(&String::from("[1,{\"c\":\"red\",\"b\":2},3]")), 4);
        assert_eq!(pt2(&String::from("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")), 0);
        assert_eq!(pt2(&String::from("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")), 0);
        assert_eq!(pt2(&String::from("{\"a:20,\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")), 0);
        assert_eq!(pt2(&String::from("[1,\"red\",5]")), 6);
    }
}