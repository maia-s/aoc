use crate::Input;
use str_block::str_block;

pub const INPUTS: &[Input] = &[
    Input::Hashed("8b278ec0816291e953d8e2e60113c4f1c1b6d76634ce8a64d9eda1f7702cfd9d"),
    Input::Inline(
        "example",
        str_block! {"
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
        "},
        Some(14),
        Some(34),
    ),
];

struct Antennae {
    map: [(u8, [(i8, i8); 4]); 0x50],
    width: u8,
    height: u8,
}

impl Default for Antennae {
    fn default() -> Self {
        Self {
            map: [(0, [(0, 0); 4]); 0x50],
            width: 0,
            height: 0,
        }
    }
}

impl Antennae {
    fn parse(input: &str) -> Self {
        let mut map = Self::default();
        for line in input.as_bytes()[..input.len() - 1].split(|&b| b == b'\n') {
            map.width = 0;
            for b in line.iter().copied() {
                if b >= b'0' {
                    let m = unsafe { map.map.get_unchecked_mut((b - b'0') as usize) };
                    m.1[m.0 as usize] = (map.width as i8, map.height as i8);
                    m.0 += 1;
                }
                map.width += 1;
            }
            map.height += 1;
        }
        map
    }

    #[inline(always)]
    fn in_range(&self, x: i8, y: i8) -> bool {
        (x as u8) < self.width && (y as u8) < self.height
    }
}

struct LocMap([u64; 0x40]);

impl Default for LocMap {
    fn default() -> Self {
        Self([0; 0x40])
    }
}

impl LocMap {
    #[inline(always)]
    fn set(&mut self, x: i8, y: i8) -> bool {
        let (i, m) = (y as usize, 1 << x as u8);
        let c = unsafe { self.0.get_unchecked_mut(i) };
        let new = *c & m == 0;
        *c |= m;
        new
    }
}

pub fn part1(input: &str) -> u32 {
    let map = Antennae::parse(input);
    let mut anti = LocMap::default();
    let mut count = 0;
    for locs in map.map {
        let locs = &locs.1[..locs.0 as usize];
        for (i, (ax, ay)) in locs.iter().copied().enumerate() {
            for (bx, by) in locs[i + 1..].iter().copied() {
                let dx = ax - bx;
                let dy = ay - by;
                let a1x = ax + dx;
                let a1y = ay + dy;
                let a2x = bx - dx;
                let a2y = by - dy;
                if map.in_range(a1x, a1y) {
                    count += anti.set(a1x, a1y) as u32;
                }
                if map.in_range(a2x, a2y) {
                    count += anti.set(a2x, a2y) as u32;
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let map = Antennae::parse(input);
    let mut anti = LocMap::default();
    let mut count = 0;
    for locs in map.map {
        let locs = &locs.1[..locs.0 as usize];
        for (i, (ax, ay)) in locs.iter().copied().enumerate() {
            for (bx, by) in locs[i + 1..].iter().copied() {
                let dx = ax - bx;
                let dy = ay - by;
                let mut hx = ax;
                let mut hy = ay;
                loop {
                    count += anti.set(hx, hy) as u32;
                    hx += dx;
                    hy += dy;
                    if !map.in_range(hx, hy) {
                        break;
                    }
                }
                let mut hx = bx;
                let mut hy = by;
                loop {
                    count += anti.set(hx, hy) as u32;
                    hx -= dx;
                    hy -= dy;
                    if !map.in_range(hx, hy) {
                        break;
                    }
                }
            }
        }
    }
    count
}
