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
