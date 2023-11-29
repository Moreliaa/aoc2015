use aoc_lib::tree;
use fancy_regex::Regex;

pub fn run(input: String) {
    println!("Day12 Pt1: {}", pt1(&input));
    println!("Day12 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let all_captures = re.captures_iter(&input);
    let mut sum = 0;
    for cap in all_captures {
        let val = cap
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        sum += val;
    }
    sum
}

struct Node {
    sum: i32,
    is_red: bool,
}

fn pt2(input: &String) -> i32 {
    let mut tree = tree::Tree::new(Node {
        sum: 0,
        is_red: false,
    });
    let mut current_node = tree.get_mut_val(0);
    let mut current_id = 0;
    let mut i: usize = 0;
    let mut is_red = false;
    let mut indent = 0;
    let mut indent_red = -1;
    let mut num: String = String::new();
    while i < input.len() {
        let sub = &input[i..i + 1];
        if i < input.len() - 6 && &input[i..i + 6] == ":\"red\"" {
            current_node.is_red = true;
            indent_red = indent;
        }
        match sub {
            "-" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                num.push_str(sub);
            }
            "{" => {
                indent += 1;
                if !is_red && num.len() > 0 {
                    current_node.sum += num.parse::<i32>().unwrap();
                }
                num = String::new();
                let parent_is_red = current_node.is_red;
                current_id = tree.add_child(
                    current_id,
                    Node {
                        sum: 0,
                        is_red: parent_is_red,
                    },
                );
                current_node = tree.get_mut_val(current_id);
            }
            "}" => {
                indent -= 1;
                if !is_red && num.len() > 0 {
                    current_node.sum += num.parse::<i32>().unwrap();
                }
                if indent < indent_red {
                    is_red = false;
                    indent_red = -1;
                }
                num = String::new();
                let next_id = tree.get_parent_id(current_id).unwrap();
                current_id = next_id;
                current_node = tree.get_mut_val(current_id);
            }
            _ => {
                if !is_red && num.len() > 0 {
                    current_node.sum += num.parse::<i32>().unwrap();
                }
                num = String::new();
            }
        }
        i += 1;
    }
    tree.aggregate_root(|id, node| {
        if node.is_red {
            return 0;
        }
        let mut parent_node = tree.get_parent_id(id);
        loop {
            match parent_node {
                Some(val) => {
                    if tree.get_val(*val).is_red {
                        return 0;
                    }
                    parent_node = tree.get_parent_id(*val);
                }
                None => break,
            };
        }
        node.sum
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(pt2(&String::from("[1,2,3]")), 6);
        assert_eq!(pt2(&String::from("[1,{\"c\":\"red\",\"b\":2},3]")), 4);
        assert_eq!(
            pt2(&String::from("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")),
            0
        );
        assert_eq!(
            pt2(&String::from("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")),
            0
        );
        assert_eq!(
            pt2(&String::from(
                "{\"a:20,\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"
            )),
            0
        );
        assert_eq!(
            pt2(&String::from(
                "[{\"a:20,\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5},\"c\":5]"
            )),
            5
        );
        assert_eq!(pt2(&String::from("[1,\"red\",5]")), 6);
    }
}
