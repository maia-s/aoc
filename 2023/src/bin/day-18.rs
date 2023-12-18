use std::{collections::BTreeMap, str::FromStr};

use aoc_2023::{aoc, str_block, Dir, Error};

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
                0 => Dir::E,
                1 => Dir::S,
                2 => Dir::W,
                3 => Dir::N,
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
            let (dx, dy) = (dx as i32, dy as i32);
            if let Some(tile) = map.0.get(&pos) {
                map.0.insert(pos, tile.rev() | dir);
            }
            if matches!(dir, Dir::N | Dir::S) {
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
            Dir::SE | Dir::SW | Dir::NW | Dir::NE => {
                map.0.insert((0, 0), dir);
            }
            Dir::NS => {
                map.0.insert((0, 0), Dir::N);
            }
            Dir::WE => {
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
                Dir::SE => {
                    if inside {
                        filled += (x - fx) as usize;
                    }
                    fx = x;
                    up = true;
                }
                Dir::NE => {
                    if inside {
                        filled += (x - fx) as usize;
                    }
                    fx = x;
                    up = false;
                }
                Dir::SW => {
                    filled += (x - fx + 1) as usize;
                    if !up {
                        inside = !inside
                    }
                    fx = x + 1;
                }
                Dir::NW => {
                    filled += (x - fx + 1) as usize;
                    if up {
                        inside = !inside
                    }
                    fx = x + 1;
                }
                Dir::N | Dir::S => {
                    filled += 1;
                    if inside {
                        filled += (x - fx) as usize;
                    }
                    inside = !inside;
                    fx = x + 1;
                }
                x => unreachable!("{:?}", x),
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
