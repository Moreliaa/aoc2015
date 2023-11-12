use itertools::Itertools;

pub fn run(input: String) {
    let mut area_total = 0;
    let mut ribbon = 0;
    for line in input.lines() {
        let (l, w, h) = line.split('x').tuples().next().unwrap();
        let (l, w, h) = (l.parse::<i32>().unwrap(), w.parse::<i32>().unwrap(), h.parse::<i32>().unwrap());

        let area_sides: Vec<i32> = vec![l * w, w * h, l * h];
        let area_package = area_sides.clone().into_iter()
            .fold(0,|acc, val| acc + val * 2);
        let min = area_sides.into_iter()
            .reduce(|acc, val| acc.min(val)).unwrap();
        area_total += area_package + min;

        ribbon += (l * w * h)
                + 2
                * (
                    l + w + h
                    - vec![l,w,h].into_iter()
                        .reduce(|acc, val| acc.max(val)).unwrap()
                );
    }

    println!("Pt1: {area_total}");
    println!("Pt2: {ribbon}");

}