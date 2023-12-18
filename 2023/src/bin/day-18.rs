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
        instructions: Vec<Instruction>,
    }

    self(input = INPUT) {
        Ok(Self { instructions: input.lines().map(str::parse).collect::<Result<Vec<Instruction>, _>>()? })
    }

    1 part1 usize {
        Ok(self.lagoon_size(|instr| (instr.dir, instr.count as isize)))
    }

    2 part2 usize {
        Ok(self.lagoon_size(|instr| (
            match instr.color.0 & 0xf {
                0 => Dir::R,
                1 => Dir::D,
                2 => Dir::L,
                3 => Dir::U,
                _ => unreachable!(),
            },
            (instr.color.0 >> 4) as isize
        )))
    }

    INPUT_EX { 1 part1 = 62, 2 part2 = 952408144115 }
    INPUT { 1 part1 = 44436 }
}

impl Day18 {
    fn lagoon_size(&self, decode: impl Fn(&Instruction) -> (Dir, isize)) -> usize {
        let (mut minx, mut maxx) = (0, 0);
        let (mut miny, mut maxy) = (0, 0);
        let mut pos = (0, 0);
        for instr in self.instructions.iter() {
            let (dir, count) = decode(instr);
            let (dx, dy) = dir.delta();
            pos.0 += dx * count;
            pos.1 += dy * count;
            minx = minx.min(pos.0);
            maxx = maxx.max(pos.0);
            miny = miny.min(pos.1);
            maxy = maxy.max(pos.1);
        }

        let width = maxx - minx + 1;
        let height = maxy - miny + 1;
        let mut map = vec![vec![Dir::NONE; width as usize]; height as usize];

        let mut pos = (-minx, -miny);
        for instr in self.instructions.iter() {
            let (dir, count) = decode(instr);
            let (dx, dy) = dir.delta();
            let tile = &mut map[pos.1 as usize][pos.0 as usize];
            *tile = tile.rev() | dir;
            for _ in 0..count {
                pos.0 += dx;
                pos.1 += dy;
                map[pos.1 as usize][pos.0 as usize] = dir;
            }
        }

        let mut filled = width as usize * height as usize;
        for row in map.iter() {
            let mut inside = false;
            let mut up = false;
            for &tile in row.iter() {
                match tile {
                    Dir::DR => up = true,
                    Dir::UR => up = false,
                    Dir::L | Dir::R => (),
                    Dir::DL => {
                        if !up {
                            inside = !inside
                        }
                    }
                    Dir::UL => {
                        if up {
                            inside = !inside
                        }
                    }
                    Dir::U | Dir::D => inside = !inside,
                    Dir::NONE => {
                        if !inside {
                            filled -= 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        filled
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
