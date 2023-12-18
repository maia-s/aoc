use std::{collections::BTreeMap, fmt::Display, ops::BitOr, str::FromStr};

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
        Ok(self.lagoon_size(|instr| (instr.dir, instr.count as i32)))
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
            (instr.color.0 >> 4) as i32
        )))
    }

    INPUT_EX { 1 part1 = 62, 2 part2 = 952408144115 }
    INPUT { 1 part1 = 44436, 2 part2 = 106941819907437 }
}

impl Day18 {
    fn lagoon_size(&self, decode: impl Fn(&Instruction) -> (Dir, i32)) -> usize {
        let mut map = Map::new();
        let mut pos = (0, 0);

        for instr in self.instructions.iter() {
            let (dir, count) = decode(instr);
            let (dx, dy) = dir.delta();
            if let Some(tile) = map.0.get(&pos) {
                map.0.insert(pos, tile.rev() | dir);
            }
            if matches!(dir, Dir::U | Dir::D) {
                for _ in 0..count - 1 {
                    pos.1 += dx;
                    pos.0 += dy;
                    map.0.insert(pos, dir);
                }
                pos.1 += dx;
                pos.0 += dy;
            } else {
                pos.1 += dx * count;
                pos.0 += dy * count;
            }
            map.0.insert(pos, dir);
        }

        let dir0 = decode(self.instructions.first().unwrap()).0;
        let dir1 = decode(self.instructions.last().unwrap()).0;
        let dir = dir0 | dir1.rev();
        match dir {
            Dir::DR | Dir::DL | Dir::UL | Dir::UR => {
                map.0.insert((0, 0), dir);
            }
            Dir::UD => {
                map.0.insert((0, 0), Dir::U);
            }
            Dir::LR => {
                map.0.remove(&(0, 0));
            }
            _ => unreachable!(),
        }

        let mut filled = 0;
        let mut py = -1;
        let mut inside = false;
        let mut up = false;
        let mut fx = 0;
        for (&(y, x), &dir) in map.0.iter() {
            if py != y {
                py = y;
                assert!(!inside);
            }
            match dir {
                Dir::DR => {
                    if inside {
                        filled += (x - fx) as usize;
                    }
                    fx = x;
                    up = true;
                }
                Dir::UR => {
                    if inside {
                        filled += (x - fx) as usize;
                    }
                    fx = x;
                    up = false;
                }
                Dir::DL => {
                    filled += (x - fx + 1) as usize;
                    if !up {
                        inside = !inside
                    }
                    fx = x + 1;
                }
                Dir::UL => {
                    filled += (x - fx + 1) as usize;
                    if up {
                        inside = !inside
                    }
                    fx = x + 1;
                }
                Dir::U | Dir::D => {
                    filled += 1;
                    if inside {
                        filled += (x - fx) as usize;
                    }
                    inside = !inside;
                    fx = x + 1;
                }
                x => unreachable!("{:#04b}", x.0),
            }
        }
        filled
    }
}

struct Map(BTreeMap<(i32, i32), Dir>);

impl Map {
    fn new() -> Self {
        Self(BTreeMap::new())
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

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Self::U => '^',
            Self::L => '<',
            Self::D => 'v',
            Self::R => '>',
            Self::DR => '┌',
            Self::DL => '┐',
            Self::UL => '┘',
            Self::UR => '└',
            _ => unreachable!(),
        };
        write!(f, "{c}")
    }
}

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

    fn delta(self) -> (i32, i32) {
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
