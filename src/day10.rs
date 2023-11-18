pub fn run() {
    let input: Vec<i32> = "1113122113".chars().map(|v| v.to_string().parse::<i32>().unwrap()).collect();
    println!("Day10 Pt1: {}", pt1_2(input.clone(), 40));
    println!("Day10 Pt2: {}", pt1_2(input, 50));
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