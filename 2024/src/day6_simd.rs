#![allow(clippy::needless_range_loop)]

use crate::{Conf, Input};
use core::{
    mem::transmute,
    ops::{Index, IndexMut},
    ptr,
    simd::prelude::*,
};
use str_block::str_block;

pub const INPUT: Conf = Conf::new(
    Input::FileHash("a820ce3b6b89ccc820f0639477e8588fd348b294933f8868232d111dc88f65be"),
    4696,
    1443,
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
    6,
);

struct Map<const PART: u8> {
    map: Vec<u8>,
    width: u32,
    gx: i32,
    gy: i32,
    dir: Dir,
    result: u32,
}

impl<const PART: u8> Map<PART> {
    #[inline(always)]
    fn height(&self) -> u32 {
        self.map.len() as u32 / self.width
    }

    #[inline(always)]
    fn coords_to_index_unchecked(&self, (x, y): (i32, i32)) -> usize {
        (y as u32 * self.width + x as u32) as usize
    }

    #[inline(always)]
    fn get(&self, (x, y): (i32, i32)) -> Option<&u8> {
        ((x as u32) < self.width && (y as u32) < self.height())
            .then(|| unsafe { self.get_unchecked((x, y)) })
    }

    #[inline(always)]
    unsafe fn get_unchecked(&self, c: (i32, i32)) -> &u8 {
        unsafe { self.map.get_unchecked(self.coords_to_index_unchecked(c)) }
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, c: (i32, i32)) -> &mut u8 {
        unsafe {
            let i = self.coords_to_index_unchecked(c);
            self.map.get_unchecked_mut(i)
        }
    }
}

impl Map<1> {
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
            result: 0,
        }
    }

    fn step(&mut self) -> bool {
        let (gx, gy) = (self.gx, self.gy);
        if self[(gx, gy)] != b'X' {
            self[(gx, gy)] = b'X';
            self.result += 1;
        }
        loop {
            let (dx, dy) = self.dir.delta();
            let next = (gx + dx, gy + dy);
            if let Some(&tile) = self.get(next) {
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

impl Map<2> {
    fn new(input: &str) -> Self {
        let mut lines = input.as_bytes().trim_ascii_end().split(|&b| b == b'\n');
        let line = lines.next().unwrap();
        let width = line.len() as _;
        let mut map = Vec::with_capacity((width * width) as usize);
        map.extend(line.iter().map(|&b| (b == b'#') as u8 * u8::MAX));
        let mut gx = 0;
        let mut gy = 1;
        let mut got_start = false;
        for line in lines.by_ref() {
            map.extend(line.iter().enumerate().map(|(x, &b)| match b {
                b'#' => u8::MAX,
                b'^' => {
                    got_start = true;
                    gx = x as i32;
                    0
                }
                _ => 0,
            }));
            if got_start {
                break;
            }
            gy += 1;
        }
        for line in lines {
            map.extend(line.iter().map(|&b| (b == b'#') as u8 * u8::MAX));
        }
        Self {
            map,
            width,
            gx,
            gy,
            dir: Dir::N,
            result: 0,
        }
    }

    fn loops64(&self, maps: [*mut u8; 64]) -> u32 {
        const OFFSETS: [usize; 64] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
        ];
        const DELTAS: [usize; 5] = [usize::MAX, 0, 1, 0, usize::MAX];
        for map in maps {
            unsafe { map.copy_from_nonoverlapping(self.map.as_ptr(), self.map.len()) };
        }
        let mut offsets = usizex64::from_array(OFFSETS);
        let map = Simd::<*mut u8, 64>::from_array(maps);
        let width = usizex64::splat(self.width as usize);
        let height = usizex64::splat(self.height() as usize);
        let mut dir = u8x64::splat(self.dir as u8);
        let mut gx = usizex64::splat(self.gx as usize);
        let mut gy = usizex64::splat(self.gy as usize);
        let vzero = u8x64::default();
        let vone = u8x64::splat(1);
        let v3 = u8x64::splat(3);
        let vblock = u8x64::splat(u8::MAX);
        let mut found = masksizex64::splat(false);

        loop {
            let active = !found & gx.simd_lt(width) & gy.simd_lt(height);
            if active.any() {
                let index = (gy * width + gx).cast();
                let tile = unsafe {
                    u8x64::gather_select_ptr(map.cast_const().wrapping_offset(index), active, vzero)
                };

                let block =
                    active.cast() & offsets.simd_eq(vzero.cast()).cast() & tile.simd_eq(vzero);
                let tile = block.select(vblock, tile);
                unsafe { tile.scatter_select_ptr(map.wrapping_offset(index), block.cast()) };
                offsets -= vone.cast();

                let dirbit = vone << dir;
                let dx = usizex64::gather_or_default(&DELTAS[1..], dir.cast::<usize>());
                let dy = usizex64::gather_or_default(&DELTAS[..4], dir.cast::<usize>());

                let wall = active & tile.cast::<i8>().simd_lt(vzero.cast()).cast::<isize>();
                let found_ = active & (tile & dirbit).simd_ne(vzero).cast::<isize>() & !wall;
                let rest = active & !wall & !found_;

                gx = wall.select(gx - dx, gx);
                gy = wall.select(gy - dy, gy);
                dir = wall.cast::<i8>().select((dir + vone) & v3, dir);

                found |= found_;

                unsafe { (tile | dirbit).scatter_select_ptr(map.wrapping_offset(index), rest) };
                gx = rest.select(gx + dx, gx);
                gy = rest.select(gy + dy, gy);
            } else {
                return found.to_bitmask().count_ones();
            }
        }
    }
}

impl<const PART: u8> Index<(i32, i32)> for Map<PART> {
    type Output = u8;

    #[inline(always)]
    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.map[self.coords_to_index_unchecked((x, y))]
    }
}

impl<const PART: u8> IndexMut<(i32, i32)> for Map<PART> {
    #[inline(always)]
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        let i = self.coords_to_index_unchecked((x, y));
        &mut self.map[i]
    }
}

impl<const PART: u8> Index<(u32, u32)> for Map<PART> {
    type Output = u8;

    #[inline(always)]
    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        &self.map[self.coords_to_index_unchecked((x as i32, y as i32))]
    }
}

impl<const PART: u8> IndexMut<(u32, u32)> for Map<PART> {
    #[inline(always)]
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        let i = self.coords_to_index_unchecked((x as i32, y as i32));
        &mut self.map[i]
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    #[inline(always)]
    const fn rotate_cw(self) -> Self {
        unsafe { transmute((self as u8 + 1) & 3) }
    }

    #[inline(always)]
    const fn delta(self) -> (i32, i32) {
        [(0, -1), (1, 0), (0, 1), (-1, 0)][self as u8 as usize]
    }

    #[inline(always)]
    const fn bit(self) -> u8 {
        1 << self as u8
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map = Map::<1>::new(input);
    while map.step() {}
    map.result
}

pub fn part2(input: &str) -> u32 {
    let mut map = Map::<2>::new(input);

    let mut scratch = vec![0; map.map.len() * 64];
    let mut maps = [ptr::null_mut(); 64];
    for i in 0..64 {
        maps[i] = unsafe { scratch.as_mut_ptr().add(i * map.map.len()) };
    }

    let mut candidates = 0;

    loop {
        candidates += map.loops64(maps);

        let (mut dx, mut dy) = map.dir.delta();
        for _ in 0..64 {
            if (map.gx as u32) < map.width && (map.gy as u32) < map.height() {
                if (*unsafe { map.get_unchecked_mut((map.gx, map.gy)) } as i8) < 0 {
                    map.gx -= dx;
                    map.gy -= dy;
                    map.dir = map.dir.rotate_cw();
                    (dx, dy) = map.dir.delta();
                } else {
                    *unsafe { map.get_unchecked_mut((map.gx, map.gy)) } |= map.dir.bit();
                    map.gx += dx;
                    map.gy += dy;
                }
            } else {
                return candidates;
            }
        }
    }
}
