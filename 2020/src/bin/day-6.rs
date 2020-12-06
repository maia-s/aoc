const INPUT: &str = include_str!("day-6.input");

#[derive(Default)]
struct Group {
    answers: [usize; 26],
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
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
                if group.answers[i] == 0 {
                    group.answers[i] = 1;
                    current += 1;
                }
            }
        }
    }
    return total + current;
}

fn part_2() -> usize {
    let mut group = Group::default();
    let mut n = 0;
    let mut total = 0;
    for line in INPUT.lines() {
        if line.is_empty() {
            total += group.answers.iter().filter(|&&i| i == n).count();
            n = 0;
            group = Group::default();
        } else {
            n += 1;
            for ch in line.chars() {
                let i = ch as usize - 'a' as usize;
                group.answers[i] += 1;
            }
        }
    }
    total += group.answers.iter().filter(|&&i| i == n).count();
    return total;
}
