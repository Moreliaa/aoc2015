use std::time::Instant;

pub fn run() {
    let start = "1113122113";
    let input: Vec<i32> = start
        .chars()
        .map(|v| v.to_string().parse::<i32>().unwrap())
        .collect();
    let t1 = Instant::now();
    println!("Day10 Pt1: {}", pt1_2(input.clone(), 40));
    println!("Day10 Pt2: {}", pt1_2(input, 50));
    let t2 = Instant::now();
    println!("Day10 {:?}", t2.duration_since(t1));
    let input = String::from(start);
    println!(
        "Day10 Pt1 (String builder): {}",
        pt1_2_bad(input.clone(), 40)
    );
    println!("Day10 Pt2 (String builder): {}", pt1_2_bad(input, 50));
    println!(
        "Day10 (String builder) {:?}",
        Instant::now().duration_since(t2)
    );
}

fn pt1_2(mut input: Vec<i32>, steps: i32) -> usize {
    let mut next: Vec<i32> = vec![];
    let mut step = 0;

    while step < steps {
        step += 1;
        let mut i = 0;
        while i < input.len() {
            let mut count = 1;
            let c = input[i];
            while i + 1 < input.len() && input[i + 1] == c {
                count += 1;
                i += 1;
            }
            next.push(count);
            next.push(c);
            i += 1;
        }
        input = next;
        next = vec![];
    }
    input.len()
}

fn pt1_2_bad(mut input: String, steps: i32) -> usize {
    let mut next = String::new();
    let mut step = 0;

    while step < steps {
        step += 1;
        let mut i = 0;
        while i < input.len() {
            let mut count = 1;
            let c = &input[i..i + 1];
            while i + 1 < input.len() && &input[i + 1..i + 2] == c {
                count += 1;
                i += 1;
            }
            next.push_str(count.to_string().as_str());
            next.push_str(c);
            i += 1;
        }
        input = next;
        next = String::new();
    }
    input.len()
}
