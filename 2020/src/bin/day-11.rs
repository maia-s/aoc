use std::{fmt, fmt::Display};

const INPUT: &str = include_str!("day-11.input");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> usize {
    let mut seats = Seats::new(INPUT);
    while seats.step(Neighbors::Immediate, 4) {}
    seats.count(Seat::Occupied)
}

fn part_2() -> usize {
    let mut seats = Seats::new(INPUT);
    while seats.step(Neighbors::Seen, 5) {}
    seats.count(Seat::Occupied)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Floor => '.',
                Seat::Empty => 'L',
                Seat::Occupied => '#',
            }
        )
    }
}

enum Neighbors {
    Immediate,
    Seen,
}

struct Seats(Vec<Vec<Seat>>);

impl Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            for &seat in row.iter() {
                write!(f, "{}", seat)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Seats {
    fn new(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|ch| match ch {
                            '.' => Seat::Floor,
                            'L' => Seat::Empty,
                            _ => panic!("'{}' in input", ch),
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn count(&self, what: Seat) -> usize {
        self.0
            .iter()
            .map(|row| row.iter().filter(|&&x| x == what).count())
            .sum()
    }

    fn seat(&self, x: usize, y: usize) -> Option<Seat> {
        self.0.get(y).and_then(|row| row.get(x).copied())
    }

    fn seen(&self, x: usize, y: usize, dx: isize, dy: isize) -> usize {
        let mut x = x as isize;
        let mut y = y as isize;
        loop {
            x += dx;
            y += dy;
            match self.seat(x as usize, y as usize) {
                Some(Seat::Empty) | None => return 0,
                Some(Seat::Occupied) => return 1,
                _ => (),
            }
        }
    }

    fn occupied_immediate(&self, x: usize, y: usize) -> usize {
        let mut n = 0;
        for j in y.saturating_sub(1)..=(y + 1) {
            if let Some(row) = self.0.get(j) {
                for i in x.saturating_sub(1)..=(x + 1) {
                    if i != x || j != y {
                        match row.get(i) {
                            Some(Seat::Occupied) => n += 1,
                            _ => (),
                        }
                    }
                }
            }
        }
        n
    }

    fn occupied_seen(&self, x: usize, y: usize) -> usize {
        self.seen(x, y, 0, -1)
            + self.seen(x, y, 1, -1)
            + self.seen(x, y, 1, 0)
            + self.seen(x, y, 1, 1)
            + self.seen(x, y, 0, 1)
            + self.seen(x, y, -1, 1)
            + self.seen(x, y, -1, 0)
            + self.seen(x, y, -1, -1)
    }

    fn step(&mut self, neighbors: Neighbors, threshold: usize) -> bool {
        let neighbors = |x, y| match neighbors {
            Neighbors::Immediate => self.occupied_immediate(x, y),
            Neighbors::Seen => self.occupied_seen(x, y),
        };
        let mut changes = vec![];
        for (y, row) in self.0.iter().enumerate() {
            for (x, &seat) in row.iter().enumerate() {
                match seat {
                    Seat::Floor => (),
                    Seat::Empty => {
                        if neighbors(x, y) == 0 {
                            changes.push((x, y, Seat::Occupied));
                        }
                    }
                    Seat::Occupied => {
                        if neighbors(x, y) >= threshold {
                            changes.push((x, y, Seat::Empty));
                        }
                    }
                }
            }
        }
        if changes.is_empty() {
            false
        } else {
            for (x, y, seat) in changes.drain(..) {
                self.0[y][x] = seat;
            }
            true
        }
    }
}
