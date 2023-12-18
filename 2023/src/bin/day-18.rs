use std::{ops::BitOr, str::FromStr};

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-18.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"};

aoc! {
    struct Day18 {
        map: Vec<Vec<Tile>>,
        width: usize,
        height: usize,
    }

    self(input = INPUT) {
        let instructions = input.lines().map(str::parse).collect::<Result<Vec<Instruction>, _>>()?;
        let (mut minx, mut maxx) = (0, 0);
        let (mut miny, mut maxy) = (0, 0);
        let mut pos = (0, 0);
        for Instruction { dir, count, .. } in instructions.iter() {
            let (dx, dy) = dir.delta();
            for _ in 0..*count {
                pos.0 += dx;
                pos.1 += dy;
            }
            minx = minx.min(pos.0);
            maxx = maxx.max(pos.0);
            miny = miny.min(pos.1);
            maxy = maxy.max(pos.1);
        }

        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
        let mut map = vec![vec![Tile::default(); width as usize]; height as usize];
        pos = (-minx, -miny);
        for Instruction { dir, count, color } in instructions.iter() {
            let (dx, dy) = dir.delta();
            let tile = &mut map[pos.1 as usize][pos.0 as usize];
            tile.color = *color;
            tile.dir = tile.dir.rev() | *dir;
            for i in 0..*count {
                pos.0 += dx;
                pos.1 += dy;
                let tile = &mut map[pos.1 as usize][pos.0 as usize];
                tile.dir = *dir;
                if i != *count - 1 {
                    tile.color = *color;
                }
            }
        }
        for row in map.iter_mut() {
            for c in row.iter_mut() {
                match c.dir {
                    Dir::U | Dir::D => c.dir = Dir::U | Dir::D,
                    Dir::L | Dir::R => c.dir = Dir::L | Dir::R,
                    _ => (),
                }
            }
        }

        Ok(Self { map, width: width as usize, height: height as usize })
    }

    1 part1 usize {
        let mut filled = self.width * self.height;
        for row in self.map.iter() {
            let mut inside = false;
            let mut up = false;
            for tile in row.iter() {
                match tile.dir {
                    Dir::DR => up = true,
                    Dir::UR => up = false,
                    Dir::LR => (),
                    Dir::DL => if !up { inside = !inside },
                    Dir::UL => if up { inside = !inside },
                    Dir::UD => inside = !inside,
                    Dir::NONE => if !inside { filled -= 1; }
                    _ => unreachable!(),
                }
            }
        }
        Ok(filled)
    }

    INPUT_EX { 1 part1 = 62 }
    INPUT { 1 part1 = 44436 }
}

#[derive(Clone, Copy)]
struct Tile {
    color: Color,
    dir: Dir,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            color: Color::NONE,
            dir: Dir::NONE,
        }
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    dir: Dir,
    count: u8,
    color: Color,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let dir = it.next().ok_or("instruction missing direction")?.parse()?;
        let count = it
            .next()
            .ok_or("instruction missing count")?
            .parse()
            .map_err(|_| "invalid count")?;
        let color = it.next().ok_or("instruction missing color")?.parse()?;
        Ok(Self { dir, count, color })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Dir(u8);

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "L" => Ok(Self::L),
            "D" => Ok(Self::D),
            "R" => Ok(Self::R),
            _ => Err("invalid direction".into()),
        }
    }
}

impl BitOr for Dir {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Dir {
    const NONE: Self = Self(0);

    const U: Self = Self(0b1000);
    const L: Self = Self(0b0100);
    const D: Self = Self(0b0010);
    const R: Self = Self(0b0001);

    const UL: Self = Self(0b1100);
    const UR: Self = Self(0b1001);
    const DL: Self = Self(0b0110);
    const DR: Self = Self(0b0011);
    const UD: Self = Self(0b1010);
    const LR: Self = Self(0b0101);

    fn rev(self) -> Self {
        Self(((self.0 | (self.0 << 4)) >> 2) & 0xf)
    }

    fn delta(self) -> (isize, isize) {
        match self {
            Self::U => (0, -1),
            Self::L => (-1, 0),
            Self::D => (0, 1),
            Self::R => (1, 0),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Color(u32);

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("(#")
            .and_then(|s| s.strip_suffix(')'))
            .ok_or("invalid color format")?;
        Ok(Self(
            u32::from_str_radix(s, 16).map_err(|_| "invalid color code")?,
        ))
    }
}

impl Color {
    const NONE: Self = Self(0xffffffff);
}
