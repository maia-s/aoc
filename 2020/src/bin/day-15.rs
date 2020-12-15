use std::collections::HashMap;

const INPUT: &str = "10,16,6,0,1,17";

fn main() {
    println!("part 1: {}", part_1());
}

fn part_1() -> usize {
    let mut spoken = HashMap::new();
    let mut turn = 1;
    let mut next = 0;

    for n in INPUT.split(",").map(|s| s.parse().unwrap()) {
        if let Some(x) = spoken.insert(n, turn) {
            next = turn - x;
        } else {
            next = 0;
        }
        turn += 1;
    }

    while turn < 2020 {
        if let Some(x) = spoken.insert(next, turn) {
            next = turn - x;
        } else {
            next = 0;
        }
        turn += 1;
    }

    next
}
