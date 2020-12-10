const INPUT: &str = include_str!("day-10.input");

fn main() {
    println!("part 1: {}", part_1());
}

fn part_1() -> isize {
    let mut ones = 0;
    let mut threes = 1;
    let mut jolts: Vec<isize> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
    jolts.push(0);
    jolts.sort_unstable();
    for i in jolts.windows(2) {
        match i[1] - i[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}
