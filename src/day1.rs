pub fn run(input: String) {
    let mut floor = 0;
    let mut pos = 0;
    for (idx, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            _ => {
                floor -= 1;
                if pos == 0 && floor == -1 {
                    pos = idx + 1;
                }
            }
        }
    }

    println!("Pt1: {floor}");
    println!("Pt2: {pos}");
}
