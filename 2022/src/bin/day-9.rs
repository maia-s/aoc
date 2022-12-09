use std::{collections::HashSet, error::Error};

const INPUT: &str = include_str!("day-9.txt");

#[cfg(test)]
const INPUT_EX: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[cfg(test)]
const INPUT_EX2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

aoc_2022::aoc! {
    struct Day9 {
        motions: Vec<Motion>,
    }

    self(input) {
        Ok(Self {
            motions: input.lines().map(|line| {
                let Some((m, n)) = line.split_once(' ') else {
                    return Err("invalid input".into());
                };
                let n: isize = n.parse()?;
                Ok(match m {
                    "U" => Motion::Vertical(-n),
                    "D" => Motion::Vertical(n),
                    "L" => Motion::Horizontal(-n),
                    "R" => Motion::Horizontal(n),
                    _ => return Err("unknown motion".into()),
                })
            }).collect::<Result<_, Box<dyn Error>>>()?
        })
    }

    part1 usize {
        let mut rope = Rope::new(1);
        for &m in self.motions.iter() {
            rope.motion(m);
        }
        Ok(rope.tail_history.len())
    }

    part2 usize {
        let mut rope = Rope::new(9);
        for &m in self.motions.iter() {
            rope.motion(m);
        }
        Ok(rope.tail_history.len())
    }

    input = INPUT;
    test day9_ex(INPUT_EX, 13, 1);
    test day9_ex2(INPUT_EX2,, 36);
    test day9(INPUT, 6563, 2653);
}

struct Rope {
    head: (isize, isize),
    tail: Vec<(isize, isize)>,
    tail_history: HashSet<(isize, isize)>,
}

impl Rope {
    fn new(len: usize) -> Self {
        let tail = vec![(0, 0); len];
        let mut tail_history = HashSet::new();
        tail_history.insert((0, 0));
        Self {
            head: (0, 0),
            tail,
            tail_history,
        }
    }

    fn motion(&mut self, motion: Motion) {
        match motion {
            Motion::Horizontal(n) => {
                if n < 0 {
                    for _ in 0..-n {
                        self.step(-1, 0);
                    }
                } else {
                    for _ in 0..n {
                        self.step(1, 0);
                    }
                }
            }
            Motion::Vertical(n) => {
                if n < 0 {
                    for _ in 0..-n {
                        self.step(0, -1);
                    }
                } else {
                    for _ in 0..n {
                        self.step(0, 1);
                    }
                }
            }
        }
    }

    fn step(&mut self, dx: isize, dy: isize) {
        self.head.0 += dx;
        self.head.1 += dy;
        let mut prev = self.head;
        for tail in self.tail.iter_mut() {
            let dx = prev.0 - tail.0;
            let dy = prev.1 - tail.1;
            if dx.abs() > 1 || dy.abs() > 1 {
                let dx = dx.clamp(-1, 1);
                let dy = dy.clamp(-1, 1);
                *tail = (tail.0 + dx, tail.1 + dy);
                prev = *tail;
            } else {
                break;
            }
        }
        self.tail_history.insert(*self.tail.last().unwrap());
    }
}

#[derive(Clone, Copy)]
enum Motion {
    Horizontal(isize),
    Vertical(isize),
}
