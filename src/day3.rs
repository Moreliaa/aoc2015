use std::collections::HashSet;

pub fn run(input: String) {
    println!("Pt1: {}", pt1(&input));
    println!("Pt2: {}", pt2(&input));
}

fn pt1(input: &str) -> usize {
    let mut set:HashSet<(i32, i32)> = HashSet::new();
    let (mut x, mut y) = (0, 0);
    set.insert((x, y));
    for c in input.chars() {
        match c {
            '^' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            'v' => y += 1,
            _ => panic!("unexpected!")
        }
        set.insert((x, y));
    }
    set.len()
}

fn pt2(input: &str) -> usize {
    let mut set:HashSet<(i32, i32)> = HashSet::new();
    struct Santa(i32, i32);
    let mut s = Santa(0, 0);
    let mut r = Santa(0, 0);

    set.insert((0, 0));
    for (idx, c) in input.chars().enumerate() {
        let m = if idx % 2 == 0 { &mut s } else { &mut r };
        match c {
            '^' => m.1 -= 1,
            '<' => m.0 -= 1,
            '>' => m.0 += 1,
            'v' => m.1 += 1,
            _ => panic!("unexpected!")
        }
        set.insert((m.0, m.1));
    }
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(pt1(">"), 2);
        assert_eq!(pt1("^>v<"), 4);
        assert_eq!(pt1("^v^v^v^v^v"), 2);
        assert_eq!(pt2("^v^v^v^v^v"), 11);
    }
}