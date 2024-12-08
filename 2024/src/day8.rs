use crate::{Conf, Input};
use core::array;
use str_block::str_block;

pub const INPUT: Conf = Conf::new(
    Input::FileHash("8b278ec0816291e953d8e2e60113c4f1c1b6d76634ce8a64d9eda1f7702cfd9d"),
    311,
    1115,
);

pub const EX: Conf = Conf::new(
    Input::Str(str_block! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "}),
    14,
    34,
);

struct LocMap([u64; 0x400]);

impl Default for LocMap {
    fn default() -> Self {
        Self([0; 0x400])
    }
}

impl LocMap {
    #[inline(always)]
    fn set(&mut self, x: i8, y: i8) -> bool {
        let i = (x as usize) << 8 | y as usize;
        let (i, m) = (i >> 6, 1 << (i & 0x3f));
        let new = (self.0[i] & m) == 0;
        self.0[i] |= m;
        new
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map: [_; 0x50] = array::from_fn(|_| Vec::new());
    let mut anti = LocMap::default();
    let mut count = 0;
    let mut width = 0;
    let mut height = 0;
    for line in input.as_bytes().trim_ascii_end().split(|&b| b == b'\n') {
        width = 0;
        for b in line.iter().copied() {
            if b >= b'0' {
                map[(b - b'0') as usize].push((width as i8, height as i8));
            }
            width += 1;
        }
        height += 1;
    }
    for locs in map {
        for (i, (ax, ay)) in locs.iter().enumerate() {
            for (bx, by) in locs[i + 1..].iter() {
                let dx = ax - bx;
                let dy = ay - by;
                let a1x = ax + dx;
                let a1y = ay + dy;
                let a2x = bx - dx;
                let a2y = by - dy;
                if (a1x as u8) < width && (a1y as u8) < height {
                    count += anti.set(a1x, a1y) as u32;
                }
                if (a2x as u8) < width && (a2y as u8) < height {
                    count += anti.set(a2x, a2y) as u32;
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let mut map: [_; 0x50] = array::from_fn(|_| Vec::new());
    let mut anti = LocMap::default();
    let mut count = 0;
    let mut width = 0;
    let mut height = 0;
    for line in input.as_bytes().trim_ascii_end().split(|&b| b == b'\n') {
        width = 0;
        for b in line.iter().copied() {
            if b >= b'0' {
                map[(b - b'0') as usize].push((width as i8, height as i8));
            }
            width += 1;
        }
        height += 1;
    }
    for locs in map {
        for (i, (ax, ay)) in locs.iter().enumerate() {
            for (bx, by) in locs[i + 1..].iter() {
                let dx = ax - bx;
                let dy = ay - by;
                let mut hx = ax + dx;
                let mut hy = ay + dy;
                while (hx as u8) < width && (hy as u8) < height {
                    hx += dx;
                    hy += dy;
                }
                hx -= dx;
                hy -= dy;
                while (hx as u8) < width && (hy as u8) < height {
                    count += anti.set(hx, hy) as u32;
                    hx -= dx;
                    hy -= dy;
                }
            }
        }
    }
    count
}
