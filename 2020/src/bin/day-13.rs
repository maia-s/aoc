const INPUT: &str = include_str!("day-13.input");

fn main() {
    let mut lines = INPUT.lines();
    let earliest = lines.next().unwrap().parse().unwrap();
    let buses: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| match x {
            "x" => Some(None),
            n => Some(Some(n.parse().unwrap())),
        })
        .collect();

    println!("part 1: {}", part_1(earliest, &buses));
    println!("part 2: {}", part_2(&buses));
}

fn part_1(earliest: usize, buses: &[Option<usize>]) -> usize {
    let min = buses
        .iter()
        .filter_map(|bus| bus.map(|bus| (bus - earliest % bus, bus)))
        .min()
        .unwrap();
    min.0 * min.1
}

fn part_2(buses: &[Option<usize>]) -> usize {
    let mut buses: Vec<_> = buses
        .iter()
        .enumerate()
        .filter_map(|(i, bus)| bus.map(|bus| (i, bus)))
        .collect();
    buses.sort_unstable_by(|a, b| a.1.cmp(&b.1).reverse());

    for i in ((buses[0].1 - buses[0].0)..).step_by(buses[0].1) {
        if buses.iter().all(|(j, bus)| (i + j) % bus == 0) {
            return i;
        }
    }

    unreachable!();
}
