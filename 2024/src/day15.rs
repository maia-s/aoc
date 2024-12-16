use core::fmt::{Display, Write};

use crate::Input;
use str_block::str_block;

pub const INPUTS: &[Input] = &[
    Input::Hashed("e61491bee7b72b533c988cdb8ec07cb4080f14a54985b293a0c837eea1193628"),
    Input::Inline(
        "example",
        str_block! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "},
        Some(10092),
        None,
    ),
    Input::Inline(
        "small example",
        str_block! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########
            
            <^^>>>vv<v>>v<<
        "},
        Some(2028),
        None,
    ),
];

struct Map {
    map: Vec<u8>,
    x: i32,
    y: i32,
    width: i32,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.width {
            for x in 0..self.width {
                f.write_char(if x == self.x && y == self.y {
                    '@'
                } else {
                    char::from(*unsafe { self.get_unchecked(x, y) })
                })?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Map {
    fn new(input: &mut &[u8]) -> Self {
        let first = input.split(|&b| b == b'\n').next().unwrap();
        let width = first.len();
        let mut x = 0;
        let mut y = 0;
        let mut map = Vec::with_capacity(width * width);
        map.extend_from_slice(first);
        let mut chunks = input[width + 1..].chunks_exact(width + 1);
        if let Some(x_) = first.iter().position(|&b| b == b'@') {
            x = x_ as _;
        } else {
            for line in chunks.by_ref() {
                let line = &line[..width];
                map.extend_from_slice(line);
                y += 1;
                if let Some(x_) = line.iter().position(|&b| b == b'@') {
                    x = x_ as _;
                    map[y * width + x] = b'.';
                    break;
                }
            }
        }
        for line in chunks {
            map.extend_from_slice(&line[..width]);
        }
        *input = &input[width * (width + 1) + 1..];
        Self {
            map,
            x: x as _,
            y: y as _,
            width: width as _,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&u8> {
        self.in_range(x, y)
            .then(|| unsafe { self.get_unchecked(x, y) })
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut u8> {
        self.in_range(x, y)
            .then(|| unsafe { self.get_unchecked_mut(x, y) })
    }

    unsafe fn get_unchecked(&self, x: i32, y: i32) -> &u8 {
        unsafe {
            self.map
                .get((y * self.width + x) as usize)
                .unwrap_unchecked()
        }
    }

    unsafe fn get_unchecked_mut(&mut self, x: i32, y: i32) -> &mut u8 {
        unsafe {
            self.map
                .get_mut((y * self.width + x) as usize)
                .unwrap_unchecked()
        }
    }

    fn in_range(&self, x: i32, y: i32) -> bool {
        x < self.width && y < self.width
    }

    fn do_move(&mut self, mov: Move) {
        let [dx, dy] = DELTAS[mov as usize];
        let (x, y) = (self.x, self.y);
        let (nx, ny) = (x + dx, y + dy);
        match self.get(nx, ny) {
            Some(b'.') => {
                self.x = nx;
                self.y = ny;
                return;
            }
            Some(b'O') => (),
            _ => return,
        }
        let (mut mx, mut my) = (nx, ny);
        for mut i in 0.. {
            (mx, my) = (mx + dx, my + dy);
            let c = self.get_mut(mx, my);
            match c {
                Some(c @ b'.') => {
                    *c = b'O';
                    while i != 0 {
                        i -= 1;
                        (mx, my) = (mx - dx, my - dy);
                        *unsafe { self.get_unchecked_mut(mx, my) } = b'O';
                    }
                    *unsafe { self.get_unchecked_mut(nx, ny) } = b'.';
                    self.x = nx;
                    self.y = ny;
                    return;
                }
                Some(b'O') => continue,
                _ => return,
            }
        }
        unreachable!()
    }

    fn calc_gps(&self) -> u32 {
        let mut gps = 0;
        for y in 1..self.width {
            for x in 1..self.width {
                if *unsafe { self.get_unchecked(x, y) } == b'O' {
                    gps += (100 * y + x) as u32;
                }
            }
        }
        gps
    }
}

const DELTAS: [[i32; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

enum Move {
    N,
    E,
    S,
    W,
}

fn next_move(input: &mut &[u8]) -> Option<Move> {
    loop {
        let b = input[0];
        *input = &input[1..];
        match b {
            b'^' => return Some(Move::N),
            b'>' => return Some(Move::E),
            b'v' => return Some(Move::S),
            b'<' => return Some(Move::W),
            _ => {
                if input.is_empty() {
                    return None;
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes();
    let mut map = Map::new(&mut input);
    while let Some(mv) = next_move(&mut input) {
        map.do_move(mv);
    }
    map.calc_gps()
}

pub fn part2(input: &str) -> u32 {
    0
}
