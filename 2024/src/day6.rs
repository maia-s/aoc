use crate::{Conf, Input};
use core::{
    fmt::{Display, Write},
    mem::transmute,
    ops::{Index, IndexMut},
};
use str_block::str_block;

pub const INPUT: Conf = Conf::new(
    Input::FileHash("a820ce3b6b89ccc820f0639477e8588fd348b294933f8868232d111dc88f65be"),
    4696,
    0,
);

pub const EX: Conf = Conf::new(
    Input::Str(str_block! {"
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
    "}),
    41,
    0,
);

struct Map {
    map: Vec<u8>,
    width: u32,
    gx: u32,
    gy: u32,
    dir: Dir,
    covered: u32,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &b) in self.map.iter().enumerate() {
            if i != 0 && i as u32 % self.width == 0 {
                f.write_char('\n')?;
            }
            f.write_char(char::from(b))?;
        }
        Ok(())
    }
}

impl Map {
    fn new(input: &str) -> Self {
        let mut lines = input.as_bytes().trim_ascii_end().split(|&b| b == b'\n');
        let line = lines.next().unwrap();
        let width = line.len() as _;
        let mut map = Vec::with_capacity((width * width) as usize);
        map.extend_from_slice(line);
        let mut gx = 0;
        let mut gy = 1;
        for line in lines.by_ref() {
            map.extend_from_slice(line);
            if let Some(start) = line.iter().position(|&b| b == b'^') {
                gx = start as _;
                break;
            }
            gy += 1;
        }
        for line in lines {
            map.extend_from_slice(line);
        }
        Self {
            map,
            width,
            gx,
            gy,
            dir: Dir::N,
            covered: 0,
        }
    }

    #[inline(always)]
    fn get(&self, (x, y): (u32, u32)) -> Option<u8> {
        self.map.get((y * self.width + x) as usize).copied()
    }

    fn step(&mut self) -> bool {
        let (gx, gy) = (self.gx, self.gy);
        if self[(gx, gy)] != b'X' {
            self[(gx, gy)] = b'X';
            self.covered += 1;
        }
        loop {
            let next = match self.dir {
                Dir::N => (gx, gy.wrapping_sub(1)),
                Dir::E => (gx.wrapping_add(1), gy),
                Dir::S => (gx, gy.wrapping_add(1)),
                Dir::W => (gx.wrapping_sub(1), gy),
            };
            if let Some(tile) = self.get(next) {
                if tile != b'#' {
                    (self.gx, self.gy) = next;
                    return true;
                }
                self.dir = self.dir.rotate_cw();
            } else {
                return false;
            }
        }
    }
}

impl Index<(u32, u32)> for Map {
    type Output = u8;

    #[inline(always)]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        &self.map[(y * self.width + x) as usize]
    }
}

impl IndexMut<(u32, u32)> for Map {
    #[inline(always)]
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        &mut self.map[(y * self.width + x) as usize]
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn rotate_cw(self) -> Self {
        unsafe { transmute((self as u8 + 1) & 3) }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map = Map::new(input);
    while map.step() {}
    map.covered
}

pub fn part2(input: &str) -> u32 {
    0
}
