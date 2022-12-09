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
        let mut rope = Rope::new();
        for &m in self.motions.iter() {
            rope.motion(m);
        }
        Ok(rope.tail_history.len())
    }

    part2 usize {
        todo!()
    }

    input = INPUT;
    test day9_ex(INPUT_EX, 13);
    test day9(INPUT, 6563);
}

struct Rope {
    head: (isize, isize),
    tail: (isize, isize),
    tail_history: HashSet<(isize, isize)>,
}

impl Rope {
    fn new() -> Self {
        let mut tail_history = HashSet::new();
        tail_history.insert((0, 0));
        Self {
            head: (0, 0),
            tail: (0, 0),
            tail_history,
        }
    }

    fn motion(&mut self, motion: Motion) {
        match motion {
            Motion::Horizontal(n) => {
                if n < 0 {
                    for _ in 0..-n {
                        self.move_left();
                    }
                } else {
                    for _ in 0..n {
                        self.move_right();
                    }
                }
            }
            Motion::Vertical(n) => {
                if n < 0 {
                    for _ in 0..-n {
                        self.move_up();
                    }
                } else {
                    for _ in 0..n {
                        self.move_down();
                    }
                }
            }
        }
    }

    fn move_left(&mut self) {
        self.head.0 -= 1;
        if self.tail.0 > self.head.0 + 1 {
            self.tail = (self.head.0 + 1, self.head.1);
        }
        self.tail_history.insert(self.tail);
    }

    fn move_right(&mut self) {
        self.head.0 += 1;
        if self.tail.0 < self.head.0 - 1 {
            self.tail = (self.head.0 - 1, self.head.1);
        }
        self.tail_history.insert(self.tail);
    }

    fn move_up(&mut self) {
        self.head.1 -= 1;
        if self.tail.1 > self.head.1 + 1 {
            self.tail = (self.head.0, self.head.1 + 1);
        }
        self.tail_history.insert(self.tail);
    }

    fn move_down(&mut self) {
        self.head.1 += 1;
        if self.tail.1 < self.head.1 - 1 {
            self.tail = (self.head.0, self.head.1 - 1);
        }
        self.tail_history.insert(self.tail);
    }
}

#[derive(Clone, Copy)]
enum Motion {
    Horizontal(isize),
    Vertical(isize),
}
