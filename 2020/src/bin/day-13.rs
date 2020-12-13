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

// chinese remainder theorem: https://www.youtube.com/watch?v=zIFehsBHB8o
fn part_2(buses: &[Option<usize>]) -> usize {
    let prod = buses.iter().fold(1, |acc, bus| acc * bus.unwrap_or(1));
    buses
        .iter()
        .enumerate()
        .filter_map(|(i, bus)| {
            bus.map(|bus| {
                let n = prod / bus;
                let mut x = 1;
                while (x * n) % bus != 1 {
                    x += 1;
                }
                (bus - i % bus) * n * x
            })
        })
        .sum::<usize>()
        % prod
}
