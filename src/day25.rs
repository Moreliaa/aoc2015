pub fn run() {
    let start: u128 = 20151125;
    let input: (u128, u128) = (2947, 3029);
    println!("Day25 Pt1: {}", pt1(start, input));
}

fn pt1(start: u128, input: (u128, u128)) -> u128 {
    let index = get_index(input);

    let mut current = start;
    for _ in 1..index {
        current = (current * 252533) % 33554393;
    }
    current
}

fn get_index(input: (u128, u128)) -> u128 {
    let mut result = input.1;
    for i in 0..input.0 + input.1 - 1 {
        result += i;
    }
    result
}
