use std::{str::FromStr, error::Error};

const INPUT: &str = include_str!("day-10.txt");

#[cfg(test)]
const INPUT_EX: &str = include_str!("day-10-ex.txt");

aoc_2022::aoc! {
    struct Day10 {
        ops: Vec<Op>,
    }

    self(input) {
        let ops = input.trim().lines().map(|line| line.parse()).collect::<Result<_,_>>()?;
        Ok(Self { ops })
    }

    part1 isize {
        let mut cpu = Cpu::new(&self.ops);
        let mut signal = 0;
        for cycle in 0..=220 {
            if cycle >= 20 && (cycle - 20) % 40 == 0 {
                eprintln!("{} {}", cycle, cpu.x);
                signal += cycle * cpu.x;
            }
            cpu.step();
        }
        Ok(signal)
    }

    part2 isize {
        todo!()
    }

    input = INPUT;
    test day10_ex(INPUT_EX, 13140);
}

struct Cpu<'a> {
    x: isize,
    next_x: isize,
    wait: usize,
    cycle: usize,
    ip: usize,
    ops: &'a [Op],
}

impl<'a> Cpu<'a> {
    fn new(ops: &'a [Op]) -> Self {
        Self {
            x: 1,
            next_x: 1,
            wait: 0,
            cycle: 0,
            ip: 0,
            ops,
        }
    }

    fn step(&mut self) {
        if self.wait == 0 && self.ip < self.ops.len() {
            self.x = self.next_x;
            match self.ops[self.ip] {
                Op::AddX(n) => {
                    self.next_x = self.x + n;
                    self.wait = 2;
                }
                Op::Noop => self.wait = 1,
            }
            self.ip += 1;
        }
        self.wait -= 1;
        self.cycle += 1;
    }
}

#[derive(Clone, Copy)]
enum Op {
    AddX(isize),
    Noop,
}

impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split_ascii_whitespace();        
        let op = s.next().ok_or("expected op")?;
        Ok(match op {
            "addx" => {
                let n = s.next().ok_or("expected int")?.parse()?;
                Op::AddX(n)
            }
            "noop" => Op::Noop,
            _ => return Err("unknown op".into()),
        })
    }
}
