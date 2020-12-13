const INPUT: &str = include_str!("day-13.input");

fn main() {
    let mut lines = INPUT.lines();
    let earliest = lines.next().unwrap().parse().unwrap();
    let buses: Vec<_> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| match x {
            "x" => None,
            n => Some(n.parse().unwrap()),
        })
        .collect();

    println!("part 1: {}", part_1(earliest, &buses));
}

fn part_1(earliest: usize, buses: &[usize]) -> usize {
    let min = buses
        .iter()
        .map(|&bus| (bus - earliest % bus, bus))
        .min()
        .unwrap();
    min.0 * min.1
}
