use crate::{Conf, Input};
use core::{array, mem};
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

pub fn part1(input: &str) -> u32 {
    let mut map: [_; 0x50] = array::from_fn(|_| Vec::new());
    let mut anti = [false; 0x10000];
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
                    let i = (a1x as u8 as usize) << 8 | a1y as u8 as usize;
                    count += !mem::replace(&mut anti[i], true) as u32;
                }
                if (a2x as u8) < width && (a2y as u8) < height {
                    let i = (a2x as u8 as usize) << 8 | a2y as u8 as usize;
                    count += !mem::replace(&mut anti[i], true) as u32;
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let mut map: [_; 0x50] = array::from_fn(|_| Vec::new());
    let mut anti = [false; 0x10000];
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
                    let i = (hx as u8 as usize) << 8 | hy as u8 as usize;
                    count += !mem::replace(&mut anti[i], true) as u32;
                    hx -= dx;
                    hy -= dy;
                }
            }
        }
    }
    count
}
