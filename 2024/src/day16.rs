use crate::Input;
use core::mem::transmute;
use std::collections::BinaryHeap;
use str_block::str_block;

pub const INPUTS: &[Input] = &[
    Input::Hashed("6b64f9910e1b588eff0ed5137b0bd506647031b2a2a6a819d7702bff31940b7a"),
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
        None,
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
        None,
    ),
];

struct Map {
    map: Vec<u8>,
    width: u32,
    dir: Dir,
    x: i32,
    y: i32,
}

impl Map {
    fn new(input: &[u8]) -> Self {
        let first = input.split(|&b| b == b'\n').next().unwrap();
        let width = first.len();
        let mut map = Vec::with_capacity(width * width);
        map.extend_from_slice(first);
        for line in input[width + 1..].chunks_exact(width + 1) {
            map.extend_from_slice(&line[..width]);
        }
        let si = map.iter().position(|&b| b == b'S').unwrap();
        map[si] = b'.';
        let x = si % width;
        let y = si / width;
        Self {
            map,
            width: width as _,
            dir: Dir::E,
            x: x as _,
            y: y as _,
        }
    }

    unsafe fn get_unchecked(&self, x: i32, y: i32) -> u8 {
        unsafe {
            *self
                .map
                .get(y as usize * self.width as usize + x as usize)
                .unwrap_unchecked()
        }
    }
}

struct Reindeer<'a> {
    map: &'a Map,
    dir: Dir,
    x: i32,
    y: i32,
    score: u32,
    won: bool,
}

impl PartialEq for Reindeer<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Reindeer<'_> {}

impl PartialOrd for Reindeer<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reindeer<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl<'a> Reindeer<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            map,
            dir: map.dir,
            x: map.x,
            y: map.y,
            score: 0,
            won: false,
        }
    }

    #[inline(always)]
    fn pos(&self) -> u32 {
        (self.dir as u32) << 16 | (self.y as u32) << 8 | (self.x as u32)
    }

    fn do_move(&self, mv: Move) -> Self {
        match mv {
            Move::Clockwise => Self {
                dir: self.dir.rotate_cw(),
                score: self.score + 1000,
                ..*self
            },
            Move::CounterClockwise => Self {
                dir: self.dir.rotate_ccw(),
                score: self.score + 1000,
                ..*self
            },
            Move::Forward => {
                let [dx, dy] = self.dir.delta();
                let [nx, ny] = [self.x + dx, self.y + dy];
                match unsafe { self.map.get_unchecked(nx, ny) } {
                    b'E' => Self {
                        score: self.score + 1,
                        won: true,
                        ..*self
                    },
                    b'.' => Self {
                        x: nx,
                        y: ny,
                        score: self.score + 1,
                        ..*self
                    },
                    _ => Self { ..*self },
                }
            }
        }
    }
}

struct BitSet([u64; 4096]);

impl Default for BitSet {
    fn default() -> Self {
        Self([0; 4096])
    }
}

impl BitSet {
    fn set(&mut self, i: u32) -> bool {
        let m = 1 << (i & 63);
        let i = i >> 6;
        let w = unsafe { self.0.get_mut(i as usize).unwrap_unchecked() };
        let was_set = *w & m != 0;
        *w |= m;
        was_set
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
    fn rotate_ccw(self) -> Self {
        unsafe { transmute((self as u8 + 3) & 3) }
    }

    fn delta(self) -> [i32; 2] {
        const DELTAS: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];
        DELTAS[self as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Forward,
    Clockwise,
    CounterClockwise,
}

pub fn part1(input: &str) -> u32 {
    let map = Map::new(input.as_bytes());
    let mut seen = BitSet::default();
    let deer_zero = Reindeer::new(&map);
    seen.set(deer_zero.pos());
    let mut queue = BinaryHeap::new();
    queue.push(deer_zero);
    while let Some(deer) = queue.pop() {
        for mv in [Move::Forward, Move::Clockwise, Move::CounterClockwise] {
            let moved = deer.do_move(mv);
            if moved.won {
                return moved.score;
            }
            if !seen.set(moved.pos()) {
                queue.push(moved);
            }
        }
    }
    unreachable!()
}

pub fn part2(input: &str) -> u32 {
    0
}
