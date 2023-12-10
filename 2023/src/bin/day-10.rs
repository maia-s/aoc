use std::{
    fmt::Debug,
    ops::{BitAnd, BitOr, Not},
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

#[allow(dead_code)]
const INPUT_EX3: &str = str_block! {"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"};

#[allow(dead_code)]
const INPUT_EX4: &str = str_block! {"
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"};

#[allow(dead_code)]
const INPUT_EX5: &str = str_block! {"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"};

#[allow(dead_code)]
const INPUT_EX6: &str = str_block! {"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"};

aoc! {
    struct Day10 {
        map: Vec<Vec<Tile>>,
        start: (isize, isize),
        marked: usize,
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

        let mut s = Self { map, start, marked: 0 };
        s.map[start.1 as usize][start.0 as usize] = s.get_missing_tile(start.0, start.1)?;
        Ok(s)
    }

    part1 usize {
        let start = self.start;
        let mut walkers = match self.tile(self.start.0, self.start.1).connections() {
            Dir::NS => [Walker::new(start.0, start.1, Dir::N), Walker::new(start.0, start.1, Dir::S)],
            Dir::NE => [Walker::new(start.0, start.1, Dir::N), Walker::new(start.0, start.1, Dir::E)],
            Dir::NW => [Walker::new(start.0, start.1, Dir::N), Walker::new(start.0, start.1, Dir::W)],
            Dir::SE => [Walker::new(start.0, start.1, Dir::S), Walker::new(start.0, start.1, Dir::E)],
            Dir::SW => [Walker::new(start.0, start.1, Dir::S), Walker::new(start.0, start.1, Dir::W)],
            Dir::EW => [Walker::new(start.0, start.1, Dir::E), Walker::new(start.0, start.1, Dir::W)],
            _ => unreachable!(),
        };
        let mut steps = 1;
        while walkers[0].pos != walkers[1].pos {
            steps += 1;
            walkers.iter_mut().for_each(|w| w.step(self));
        }
        Ok(steps)
    }

    part2 usize {
        let mut new_map: Vec<Vec<_>> = self.map.iter().map(
            |row| row.iter().map(|_| Tile(b'.')).collect()
        ).collect();
        let mut w = match self.tile(self.start.0, self.start.1).connections() {
            Dir::NS => Walker::new(self.start.0, self.start.1, Dir::N),
            Dir::NE => Walker::new(self.start.0, self.start.1, Dir::N),
            Dir::NW => Walker::new(self.start.0, self.start.1, Dir::N),
            Dir::SE => Walker::new(self.start.0, self.start.1, Dir::E),
            Dir::SW => Walker::new(self.start.0, self.start.1, Dir::W),
            Dir::EW => Walker::new(self.start.0, self.start.1, Dir::E),
            _ => unreachable!(),
        };
        let mut nwcpos = (isize::MAX, isize::MAX);
        while new_map[w.pos.1 as usize][w.pos.0 as usize].0 == b'.' {
            let tile = self.tile(w.pos.0, w.pos.1);
            if tile.0 == b'F' && w.pos < nwcpos {
                nwcpos = w.pos;
            }
            new_map[w.pos.1 as usize][w.pos.0 as usize] = tile;
            w.step(self);
        }
        self.map = new_map;

        let mut w = Walker::new(nwcpos.0, nwcpos.1, Dir::E);
        while w.pos != nwcpos {
            let (x, y) = w.pos;
            match (self.tile(x, y).0, w.pdir) {
                (b'|', Dir::N) => self.mark(x + 1, y),
                (b'|', Dir::S) => self.mark(x - 1, y),
                (b'-', Dir::E) => self.mark(x, y + 1),
                (b'-', Dir::W) => self.mark(x, y - 1),
                (b'F', Dir::N) => (),
                (b'F', Dir::W) => {
                    self.mark(x, y - 1);
                    self.mark(x - 1, y - 1);
                    self.mark(x - 1, y);
                }
                (b'7', Dir::E) => (),
                (b'7', Dir::N) => {
                    self.mark(x + 1, y);
                    self.mark(x + 1, y - 1);
                    self.mark(x, y - 1);
                }
                (b'J', Dir::S) => (),
                (b'J', Dir::E) => {
                    self.mark(x, y + 1);
                    self.mark(x + 1, y + 1);
                    self.mark(x + 1, y);
                }
                (b'L', Dir::W) => (),
                (b'L', Dir::S) => {
                    self.mark(x - 1, y);
                    self.mark(x - 1, y + 1);
                    self.mark(x, y + 1);
                }
                _ => unreachable!(),
            }
            w.step(self);
        }

        let mut prev_marked = 0;
        while prev_marked != self.marked {
            prev_marked = self.marked;
            for y in 0..self.map.len() as isize {
                for x in 0..self.map[0].len() as isize {
                    if self.map[y as usize][x as usize].0 == b'.' &&
                        (self.tile(x, y - 1).0 == b'I' ||
                        self.tile(x - 1, y).0 == b'I' ||
                        self.tile(x + 1, y).0 == b'I' ||
                        self.tile(x, y + 1).0 == b'I')
                    {
                        self.map[y as usize][x as usize].0 = b'I';
                        self.marked += 1;
                    }
                }
            }
        }

        Ok(self.marked)
    }

    test day10_example(INPUT_EX, 4);
    test day10_example2(INPUT_EX2, 8);
    test day10_example3(INPUT_EX3,, 4);
    test day10_example4(INPUT_EX4,, 4);
    test day10_example5(INPUT_EX5,, 8);
    test day10_example6(INPUT_EX6,, 10);
    test day10(INPUT, 6599, 477);
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

    fn mark(&mut self, x: isize, y: isize) {
        let x = x as usize;
        let y = y as usize;
        if self.map.get(y).and_then(|row| row.get(x).copied()) == Some(Tile(b'.')) {
            self.map[y][x].0 = b'I';
            self.marked += 1;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Tile(u8);

impl Tile {
    fn connections(&self) -> Dir {
        (*self).into()
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self.0))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Dir(u8);

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04b}", self.0)
    }
}

impl From<Tile> for Dir {
    fn from(value: Tile) -> Self {
        match value.0 {
            b'|' => Dir::NS,
            b'L' => Dir::NE,
            b'J' => Dir::NW,
            b'F' => Dir::SE,
            b'7' => Dir::SW,
            b'-' => Dir::EW,
            _ => Dir(0),
        }
    }
}

impl Not for Dir {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(self.0 ^ 0x0f)
    }
}

impl BitAnd for Dir {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Dir {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Dir {
    const N: Self = Self(0b1000);
    const S: Self = Self(0b0100);
    const E: Self = Self(0b0010);
    const W: Self = Self(0b0001);

    const NS: Self = Self(0b1100);
    const NE: Self = Self(0b1010);
    const NW: Self = Self(0b1001);
    const SE: Self = Self(0b0110);
    const SW: Self = Self(0b0101);
    const EW: Self = Self(0b0011);

    fn rev(&self) -> Self {
        let a = self.0 & 0b1010;
        let b = self.0 & 0b0101;
        Self(a >> 1 | b << 1)
    }

    fn n(self) -> bool {
        (self & Self::N).0 != 0
    }

    fn s(self) -> bool {
        (self & Self::S).0 != 0
    }

    fn e(self) -> bool {
        (self & Self::E).0 != 0
    }

    fn w(self) -> bool {
        (self & Self::W).0 != 0
    }

    fn dx(&self) -> isize {
        match self.0 & 3 {
            0b10 => 1,
            0b01 => -1,
            _ => 0,
        }
    }

    fn dy(&self) -> isize {
        match self.0 & 0xc {
            0b1000 => -1,
            0b0100 => 1,
            _ => 0,
        }
    }
}

struct Walker {
    pos: (isize, isize),
    pdir: Dir,
}

impl Walker {
    fn new(x: isize, y: isize, dir: Dir) -> Self {
        assert!(dir.0.count_ones() == 1);
        Self {
            pos: (x + dir.dx(), y + dir.dy()),
            pdir: dir,
        }
    }

    fn step(&mut self, day: &Day10) {
        let tc = day.tile(self.pos.0, self.pos.1).connections();
        self.pdir = tc & !self.pdir.rev();
        self.pos.0 += self.pdir.dx();
        self.pos.1 += self.pdir.dy();
    }
}
