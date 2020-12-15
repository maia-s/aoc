use std::collections::HashMap;

const INPUT: &str = "10,16,6,0,1,17";

fn main() {
    let mut mem = Memory::new();
    println!("part 1: {}", mem.step_until(2020));
    println!("part 2: {}", mem.step_until(30000000));
}

struct Memory {
    spoken: HashMap<usize, usize>,
    turn: usize,
    next: usize,
}

impl Memory {
    fn new() -> Self {
        let mut mem = Self {
            spoken: HashMap::new(),
            turn: 1,
            next: 0,
        };

        for n in INPUT.split(",").map(|s| s.parse().unwrap()) {
            mem.next = n;
            mem.step();
        }

        mem
    }

    fn step(&mut self) {
        self.next = if let Some(prev) = self.spoken.insert(self.next, self.turn) {
            self.turn - prev
        } else {
            0
        };
        self.turn += 1;
    }

    fn step_until(&mut self, until: usize) -> usize {
        while self.turn < until {
            self.step();
        }
        self.next
    }
}
