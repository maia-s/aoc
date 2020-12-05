const INPUT: &'static str = include_str!("day-5.input");

fn id(line: &str) -> usize {
    let mut id = 0;
    for ch in line.chars() {
        id = id * 2
            + match ch {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!(),
            };
    }
    id
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> usize {
    let mut max = 0;
    for line in INPUT.lines() {
        let id = id(line);
        if id > max {
            max = id;
        }
    }
    max
}

fn part_2() -> usize {
    let mut seats = vec![];
    for line in INPUT.lines() {
        let id = id(line);
        if id >= seats.len() {
            seats.resize(id + 1, false);
        }
        seats[id] = true;
    }
    let mut it = seats.iter().enumerate();
    while !*it.next().unwrap().1 {}
    loop {
        let (i, taken) = it.next().unwrap();
        if !taken {
            break i;
        }
    }
}
