pub fn run() {
    let target: u32 = 36000000;
    println!("Day20 Pt1: {}", pt1(target));
    println!("Day20 Pt2: {}", pt2(target));
}

fn pt1(target: u32) -> usize {
    let size = (target / 20) as usize;
    let mut storage:Vec<u32> = vec![0; size];
    for elf in 1..size {
        let mut house = elf;
        while house < size {
            storage[house] = storage[house] + (elf as u32 * 10);
            house += elf;
        }
    }

    for (house, presents) in storage.into_iter().enumerate() {
        if presents >= target {
            return house;
        }
    }

    panic!();
}

fn pt2(target: u32) -> usize {
    let size = (target / 20) as usize;
    let mut homes:Vec<u32> = vec![0; size];
    for elf in 1..size {
        let mut h = elf;
        while h < size && h < elf + elf * 50 {
            homes[h] = homes[h] + (elf as u32 * 11);
            h += elf;
        }
    }

    for (h, presents) in homes.into_iter().enumerate() {
        if presents >= target {
            return h;
        }
    }

    panic!();
}