use aoc_lib::map2d::Map2D;

pub fn run(input: String) {
    println!("Day18 Pt1: {}", pt1(&input));
    println!("Day18 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let mut map = Map2D::from_string(input.clone());
    let max_steps = 100;
    let mut steps = 0;
    while steps < max_steps {
        let mut next_map = Map2D::new(map.width(), map.height(), '#');
        for x in 0..map.width() {
            for y in 0..map.height() {
                let was_on = *map.get(x, y).unwrap() == '#';
                let coords: [(i32, i32); 8] = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];
                let on_count = coords.into_iter().fold(0, |a, b| {
                    let x1: i32 = x as i32 + b.0;
                    let y1: i32 = y as i32 + b.1;
                    a + match map.is_in_bounds(x1, y1) {
                        true => {
                            if *map.get(x1 as usize, y1 as usize).unwrap() == '#' {
                                1
                            } else {
                                0
                            }
                        }
                        false => 0,
                    }
                });
                let mut next_char = '.';
                if was_on {
                    if on_count == 2 || on_count == 3 {
                        next_char = '#';
                    }
                } else {
                    if on_count == 3 {
                        next_char = '#';
                    }
                }
                next_map.set(x, y, next_char);
            }
        }
        map = next_map;
        steps += 1;
    }
    map.aggregate(|t| if *t == '#' { 1 } else { 0 })
}

fn pt2(input: &String) -> i32 {
    let mut map = Map2D::from_string(input.clone());
    map.set(0, 0, '#');
    map.set(0, map.height() - 1, '#');
    map.set(map.width() - 1, 0, '#');
    map.set(map.width() - 1, map.height() - 1, '#');
    let max_steps = 100;
    let mut steps = 0;
    while steps < max_steps {
        let mut next_map = Map2D::new(map.width(), map.height(), '#');
        for x in 0..map.width() {
            for y in 0..map.height() {
                if (x == 0 && y == 0)
                    || (x == 0 && y == map.height() - 1)
                    || (x == map.width() - 1 && y == 0)
                    || (x == map.width() - 1 && y == map.height() - 1)
                {
                    continue;
                }
                let was_on = *map.get(x, y).unwrap() == '#';
                let coords: [(i32, i32); 8] = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];
                let on_count = coords.into_iter().fold(0, |a, b| {
                    let x1: i32 = x as i32 + b.0;
                    let y1: i32 = y as i32 + b.1;
                    a + match map.is_in_bounds(x1, y1) {
                        true => {
                            if *map.get(x1 as usize, y1 as usize).unwrap() == '#' {
                                1
                            } else {
                                0
                            }
                        }
                        false => 0,
                    }
                });
                let mut next_char = '.';
                if was_on {
                    if on_count == 2 || on_count == 3 {
                        next_char = '#';
                    }
                } else {
                    if on_count == 3 {
                        next_char = '#';
                    }
                }
                next_map.set(x, y, next_char);
            }
        }
        map = next_map;
        steps += 1;
    }
    map.aggregate(|t| if *t == '#' { 1 } else { 0 })
}
