use std::fmt::Debug;

use aoc_2023::{aoc, str_block, Dir, Error};

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

    1 part1 usize {
        let start = self.start;
        let mut walkers = match self.tile(self.start.0, self.start.1).connections() {
            Dir::NS => [Walker::new(start.0, start.1, Dir::N), Walker::new(start.0, start.1, Dir::S)],
            Dir::NE => [Walker::new(start.0, start.1, Dir::N), Walker::new(start.0, start.1, Dir::E)],
            Dir::NW => [Walker::new(start.0, start.1, Dir::N), Walker::new(start.0, start.1, Dir::W)],
            Dir::SE => [Walker::new(start.0, start.1, Dir::S), Walker::new(start.0, start.1, Dir::E)],
            Dir::SW => [Walker::new(start.0, start.1, Dir::S), Walker::new(start.0, start.1, Dir::W)],
            Dir::WE => [Walker::new(start.0, start.1, Dir::E), Walker::new(start.0, start.1, Dir::W)],
            _ => unreachable!(),
        };
        let mut steps = 1;
        while walkers[0].pos != walkers[1].pos {
            steps += 1;
            walkers.iter_mut().for_each(|w| w.step(self));
        }
        Ok(steps)
    }

    2 part2 usize {
        let mut new_map: Vec<Vec<_>> = self.map.iter().map(
            |row| row.iter().map(|_| Tile(b'.')).collect()
        ).collect();
        let mut w = match self.tile(self.start.0, self.start.1).connections() {
            Dir::NS => Walker::new(self.start.0, self.start.1, Dir::N),
            Dir::NE => Walker::new(self.start.0, self.start.1, Dir::N),
            Dir::NW => Walker::new(self.start.0, self.start.1, Dir::N),
            Dir::SE => Walker::new(self.start.0, self.start.1, Dir::E),
            Dir::SW => Walker::new(self.start.0, self.start.1, Dir::W),
            Dir::WE => Walker::new(self.start.0, self.start.1, Dir::E),
            _ => unreachable!(),
        };
        while new_map[w.pos.1 as usize][w.pos.0 as usize].0 == b'.' {
            let tile = self.tile(w.pos.0, w.pos.1);
            new_map[w.pos.1 as usize][w.pos.0 as usize] = tile;
            w.step(self);
        }
        self.map = new_map;

        let mut is_in = false;
        let mut from_s = false;
        let mut marked = 0;
        for row in self.map.iter() {
            for tile in row.iter() {
                match tile.0 {
                    b'F' => from_s = true,
                    b'L' => from_s = false,
                    b'|' => is_in = !is_in,
                    b'7' => if !from_s { is_in = !is_in },
                    b'J' => if from_s { is_in = !is_in },
                    b'.' => if is_in { marked += 1 },
                    _ => (),
                }
            }
        }

        Ok(marked)
    }

    INPUT_EX { 1 part1 = 4 }
    INPUT_EX2 { 1 part1 = 8 }
    INPUT_EX3 { 2 part2 = 4 }
    INPUT_EX4 { 2 part2 = 4 }
    INPUT_EX5 { 2 part2 = 8 }
    INPUT_EX6 { 2 part2 = 10 }
    INPUT { 1 part1 = 6599, 2 part2 = 477 }
}

impl Day10 {
    fn tile(&self, x: isize, y: isize) -> Tile {
        self.map
            .get(y as usize)
            .and_then(|row| row.get(x as usize).copied())
            .unwrap_or(Tile(b'.'))
    }

    fn get_missing_tile(&self, x: isize, y: isize) -> Result<Tile, Error> {
        let scn = self.tile(x, y - 1).connections().has_s();
        let scs = self.tile(x, y + 1).connections().has_n();
        let sce = self.tile(x + 1, y).connections().has_w();
        let scw = self.tile(x - 1, y).connections().has_e();
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

impl From<Tile> for Dir {
    fn from(value: Tile) -> Self {
        match value.0 {
            b'|' => Dir::NS,
            b'L' => Dir::NE,
            b'J' => Dir::NW,
            b'F' => Dir::SE,
            b'7' => Dir::SW,
            b'-' => Dir::WE,
            _ => Dir::NONE,
        }
    }
}

struct Walker {
    pos: (isize, isize),
    pdir: Dir,
}

impl Walker {
    fn new(x: isize, y: isize, dir: Dir) -> Self {
        assert!(dir.count_dirs() == 1);
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
