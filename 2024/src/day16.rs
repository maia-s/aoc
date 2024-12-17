use crate::Input;
use core::{
    cell::RefCell,
    fmt::{Display, Write},
    mem::transmute,
};
use std::{collections::BinaryHeap, rc::Rc};
use str_block::str_block;

pub fn inputs() -> Vec<Input> {
    vec![
        Input::Hashed("6b64f9910e1b588eff0ed5137b0bd506647031b2a2a6a819d7702bff31940b7a"),
        Input::Hashed("12de1ded54ee31faa2e37f826e4f9a0f413bb3967570ff768639d9c0724e4ab7"),
        Input::Hashed("e0347f5691af53cdd16144f94424682ac80804d2148b2d6073967f70ea1f3d66"),
        Input::Hashed("71e107ecb7332c63581077a4a9beaa5c9ec7db0a512a0d760bd753950b9e24a5"),
        Input::Inline(
            "example",
            str_block! {"
                ###############
                #.......#....E#
                #.#.###.#.###.#
                #.....#.#...#.#
                #.###.#####.#.#
                #.#.#.......#.#
                #.#.#####.###.#
                #...........#.#
                ###.#.#####.#.#
                #...#.....#.#.#
                #.#.#.###.#.#.#
                #.....#...#.#.#
                #.###.#.#.#.#.#
                #S..#.....#...#
                ###############
            "},
            Some(7036),
            Some(45),
        ),
        Input::Inline(
            "second example",
            str_block! {"
                #################
                #...#...#...#..E#
                #.#.#.#.#.#.#.#.#
                #.#.#.#...#...#.#
                #.#.#.#.###.#.#.#
                #...#.#.#.....#.#
                #.#.#.#.#.#####.#
                #.#...#.#.#.....#
                #.#.#####.#.###.#
                #.#.#.......#...#
                #.#.###.#####.###
                #.#.#...#.....#.#
                #.#.#.#####.###.#
                #.#.#.........#.#
                #.#.#.#########.#
                #S#.............#
                #################
            "},
            Some(11048),
            Some(64),
        ),
    ]
}

struct Map<const N: usize> {
    map: Vec<[u32; N]>,
    width: u32,
    dir: Dir,
    x: i32,
    y: i32,
    sx: i32,
    sy: i32,
    ex: i32,
    ey: i32,
}

impl<const N: usize> Display for Map<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.width {
            for x in 0..self.width {
                if x != 0 {
                    f.write_char(' ')?;
                }
                f.write_char('[')?;
                for n in 0..N {
                    if n != 0 {
                        f.write_char(',')?;
                    }
                    write!(f, "{:4}", self.map[(y * self.width + x) as usize][n] as i32)?;
                }
                f.write_char(']')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<const N: usize> Map<N> {
    fn new(input: &[u8]) -> Self {
        let width = input.iter().position(|&b| b == b'\n').unwrap();
        let mut map = Vec::with_capacity(width * width);
        let mut sx = 0;
        let mut sy = 0;
        let mut ex = 0;
        let mut ey = 0;
        for (y, line) in input.chunks_exact(width + 1).enumerate() {
            for x in 0..width {
                match unsafe { line.get(x).unwrap_unchecked() } {
                    b'S' => {
                        sx = x;
                        sy = y;
                        map.push([0; N]);
                    }
                    b'E' => {
                        ex = x;
                        ey = y;
                        map.push([u32::MAX; N]);
                    }
                    b'.' => map.push([u32::MAX; N]),
                    _ => map.push([0; N]),
                }
            }
        }
        Self {
            map,
            width: width as _,
            dir: Dir::E,
            x: sx as _,
            y: sy as _,
            sx: sx as _,
            sy: sy as _,
            ex: ex as _,
            ey: ey as _,
        }
    }

    unsafe fn get_unchecked(&self, x: i32, y: i32) -> &[u32; N] {
        unsafe {
            self.map
                .get(y as usize * self.width as usize + x as usize)
                .unwrap_unchecked()
        }
    }

    unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32) -> &mut [u32; N] {
        unsafe {
            self.map
                .get_mut(y as usize * self.width as usize + x as usize)
                .unwrap_unchecked()
        }
    }
}

#[derive(Clone)]
struct Reindeer<const N: usize> {
    map: Rc<RefCell<Map<N>>>,
    dir: Dir,
    x: i32,
    y: i32,
    score: u32,
    won: bool,
}

impl<const N: usize> PartialEq for Reindeer<N> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<const N: usize> Eq for Reindeer<N> {}

impl<const N: usize> PartialOrd for Reindeer<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for Reindeer<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl<const N: usize> Reindeer<N> {
    fn new(map: Rc<RefCell<Map<N>>>) -> Self {
        let m = map.borrow();
        let dir = m.dir;
        let x = m.x;
        let y = m.y;
        drop(m);
        Self {
            map,
            dir,
            x,
            y,
            score: 0,
            won: false,
        }
    }

    fn forward(&self) -> Option<Self> {
        let [dx, dy] = self.dir.delta();
        let [nx, ny] = [self.x + dx, self.y + dy];
        let mut m = self.map.borrow_mut();
        let ns = unsafe { m.get_unchecked_mut(nx, ny) };
        let n = if N == 1 {
            0
        } else if N == 4 {
            self.dir as usize
        } else {
            panic!()
        };
        if ns[n] > self.score {
            ns[n] = self.score + 1;
            let won = nx == m.ex && ny == m.ey;
            Some(Self {
                x: nx,
                y: ny,
                score: self.score + 1,
                won,
                ..self.clone()
            })
        } else {
            None
        }
    }
}

#[repr(u8)]
#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    #[must_use]
    #[inline(always)]
    fn rotate_cw(self) -> Self {
        unsafe { transmute((self as u8 + 1) & 3) }
    }

    #[must_use]
    #[inline(always)]
    fn rotate_180(self) -> Self {
        unsafe { transmute((self as u8 + 2) & 3) }
    }

    fn delta(self) -> [i32; 2] {
        const DELTAS: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];
        DELTAS[self as usize]
    }
}

#[derive(Clone, Copy)]
struct BitSet<const N: usize>([u64; N]);

impl<const N: usize> Default for BitSet<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> BitSet<N> {
    fn set(&mut self, i: u32) -> bool {
        let m = 1 << (i & 63);
        let i = i >> 6;
        let w = unsafe { self.0.get_mut(i as usize).unwrap_unchecked() };
        let was_set = *w & m != 0;
        *w |= m;
        was_set
    }
}

pub fn part1(input: &str) -> u32 {
    let mut queue = BinaryHeap::new();
    queue.push(Reindeer::<1>::new(Rc::new(RefCell::new(Map::new(
        input.as_bytes(),
    )))));
    while let Some(mut deer) = queue.pop() {
        if let Some(moved) = deer.forward() {
            if moved.won {
                return moved.score;
            }
            queue.push(moved);
        }
        deer.dir = deer.dir.rotate_cw();
        deer.score += 1000;
        if let Some(moved) = deer.forward() {
            if moved.won {
                return moved.score;
            }
            queue.push(moved);
        }
        deer.dir = deer.dir.rotate_180();
        if let Some(moved) = deer.forward() {
            if moved.won {
                return moved.score;
            }
            queue.push(moved);
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> u32 {
    let mut queue = BinaryHeap::new();
    let map = Rc::new(RefCell::new(Map::new(input.as_bytes())));
    queue.push(Reindeer::<4>::new(map.clone()));
    let mut score = u32::MAX;
    while let Some(mut deer) = queue.pop() {
        if let Some(moved) = deer.forward() {
            if score >= moved.score {
                if moved.won {
                    score = moved.score;
                } else {
                    queue.push(moved);
                }
            }
        }
        deer.dir = deer.dir.rotate_cw();
        deer.score += 1000;
        if let Some(moved) = deer.forward() {
            if score >= moved.score {
                if moved.won {
                    score = moved.score;
                } else {
                    queue.push(moved);
                }
            }
        }
        deer.dir = deer.dir.rotate_180();
        if let Some(moved) = deer.forward() {
            if score >= moved.score {
                if moved.won {
                    score = moved.score;
                } else {
                    queue.push(moved);
                }
            }
        }
    }
    let map = map.borrow();
    let mut queue = Vec::new();
    let mut seen = BitSet::<1024>::default();
    let mut seats = 0;
    let e = unsafe { map.get_unchecked(map.ex, map.ey) };
    for i in 0_u8..4 {
        if e[i as usize] == score {
            queue.push((map.ex, map.ey, unsafe { transmute::<u8, Dir>(i) }));
        }
    }
    while let Some((x, y, dir)) = queue.pop() {
        seats += !seen.set((y as u32) << 8 | x as u32) as u32;
        if x != map.sx || y != map.sy {
            let score = unsafe { map.get_unchecked(x, y) }[dir as usize];
            let [dx, dy] = dir.delta();
            let (nx, ny) = (x - dx, y - dy);
            let n = unsafe { map.get_unchecked(nx, ny) };
            if n[dir as usize] == score - 1 {
                queue.push((nx, ny, dir));
            }
            let dir = dir.rotate_cw();
            if n[dir as usize] == score.wrapping_sub(1001) {
                queue.push((nx, ny, dir));
            }
            let dir = dir.rotate_180();
            if n[dir as usize] == score.wrapping_sub(1001) {
                queue.push((nx, ny, dir));
            }
        }
    }
    seats
}
