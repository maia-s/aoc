const INPUT: &str = include_str!("day-6.input");

#[derive(Default)]
struct Group {
    answers: [bool; 26],
}

fn main() {
    println!("part 1: {}", part_1());
}

fn part_1() -> usize {
    let mut group = Group::default();
    let mut current = 0;
    let mut total = 0;
    for line in INPUT.lines() {
        if line.is_empty() {
            total += current;
            current = 0;
            group = Group::default();
        } else {
            for ch in line.chars() {
                let i = ch as usize - 'a' as usize;
                if !group.answers[i] {
                    group.answers[i] = true;
                    current += 1;
                }
            }
        }
    }
    return total + current;
}
