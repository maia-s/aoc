use std::{
    fmt::Debug,
    ops::{BitAnd, Not},
};

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-10.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
.....
.S-7.
.|.|.
.L-J.
.....
"};

#[allow(dead_code)]
const INPUT_EX2: &str = str_block! {"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"};

aoc! {
    struct Day10 {
        map: Vec<Vec<Tile>>,
        start: (isize, isize),
    }

    self(input = INPUT) {
        let mut start = (0, 0);
        let map: Vec<_> = input.lines().enumerate().map(|(y, line)| {
            let line: Vec<_> = line.as_bytes().iter().map(|&b| Tile(b)).collect();
            if let Some(x) = line.iter().position(|c| c.0 == b'S') {
                start = (x as isize, y as isize);
            }
            line
        }).collect();

        let cols = map[0].len();
        if !map.iter().all(|row| row.len() == cols) {
            return Err("all lines aren't the same length".into());
        }

        let mut s = Self { map, start };
        s.map[start.1 as usize][start.0 as usize] = s.get_missing_tile(start.0, start.1)?;
        Ok(s)
    }

    part1 usize {
        let (mut pos, mut pc) = match self.tile(self.start.0, self.start.1).connections().0 {
            0b1100 => ([(self.start.0, self.start.1 - 1), (self.start.0, self.start.1 + 1 )],
                        [Connections::N, Connections::S]),
            0b1010 => ([(self.start.0, self.start.1 - 1), (self.start.0 + 1, self.start.1 )],
                        [Connections::N, Connections::E]),
            0b1001 => ([(self.start.0, self.start.1 - 1), (self.start.0 - 1, self.start.1 )],
                        [Connections::N, Connections::W]),
            0b0110 => ([(self.start.0, self.start.1 + 1), (self.start.0 + 1, self.start.1 )],
                        [Connections::S, Connections::E]),
            0b0101 => ([(self.start.0, self.start.1 + 1), (self.start.0 - 1, self.start.1 )],
                        [Connections::S, Connections::W]),
            0b0011 => ([(self.start.0 + 1, self.start.1), (self.start.0 - 1, self.start.1 )],
                        [Connections::E, Connections::W]),
            _ => unreachable!(),
        };
        let mut steps = 1;
        while pos[0] != pos[1] {
            steps += 1;
            for (p, pc) in pos.iter_mut().zip(pc.iter_mut()) {
                let tc = self.tile(p.0, p.1).connections();
                *pc = match (tc & !pc.rev()).0 {
                    0b1000 => { p.1 -= 1; Connections::N }
                    0b0100 => { p.1 += 1; Connections::S }
                    0b0010 => { p.0 += 1; Connections::E }
                    0b0001 => { p.0 -= 1; Connections::W }
                    _ => unreachable!(),
                };
            }
        }
        Ok(steps)
    }

    test day10_example(INPUT_EX, 4);
    test day10_example2(INPUT_EX2, 8);
    test day10(INPUT, 6599);
}

impl Day10 {
    fn tile(&self, x: isize, y: isize) -> Tile {
        self.map
            .get(y as usize)
            .and_then(|row| row.get(x as usize).copied())
            .unwrap_or(Tile(b'.'))
    }

    fn get_missing_tile(&self, x: isize, y: isize) -> Result<Tile, Error> {
        let scn = self.tile(x, y - 1).connections().s();
        let scs = self.tile(x, y + 1).connections().n();
        let sce = self.tile(x + 1, y).connections().w();
        let scw = self.tile(x - 1, y).connections().e();
        match (scn, scs, sce, scw) {
            (true, true, false, false) => Ok(Tile(b'|')),
            (true, false, true, false) => Ok(Tile(b'L')),
            (true, false, false, true) => Ok(Tile(b'J')),
            (false, true, true, false) => Ok(Tile(b'F')),
            (false, true, false, true) => Ok(Tile(b'7')),
            (false, false, true, true) => Ok(Tile(b'-')),
            _ => Err(format!("tile at ({x}, {y}) doesn't have exactly two connections").into()),
        }
    }
}

#[derive(Clone, Copy)]
struct Tile(u8);

impl Tile {
    fn connections(&self) -> Connections {
        (*self).into()
    }
}

#[derive(Clone, Copy)]
struct Connections(u8);

impl Debug for Connections {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04b}", self.0)
    }
}

impl From<Tile> for Connections {
    fn from(value: Tile) -> Self {
        Self(match value.0 {
            b'|' => 0b1100,
            b'L' => 0b1010,
            b'J' => 0b1001,
            b'F' => 0b0110,
            b'7' => 0b0101,
            b'-' => 0b0011,
            _ => 0,
        })
    }
}

impl Not for Connections {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0 ^ 0x0f)
    }
}

impl BitAnd for Connections {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl Connections {
    const N: Self = Self(0b1000);
    const S: Self = Self(0b0100);
    const E: Self = Self(0b0010);
    const W: Self = Self(0b0001);

    fn rev(&self) -> Self {
        let a = self.0 & 0b1010;
        let b = self.0 & 0b0101;
        Self(a >> 1 | b << 1)
    }

    fn n(&self) -> bool {
        self.0 & 0b1000 != 0
    }

    fn s(&self) -> bool {
        self.0 & 0b0100 != 0
    }

    fn e(&self) -> bool {
        self.0 & 0b0010 != 0
    }

    fn w(&self) -> bool {
        self.0 & 0b0001 != 0
    }
}
