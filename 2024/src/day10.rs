use crate::Input;
use core::hint::unreachable_unchecked;
use str_block::str_block;

pub const INPUTS: &[Input] = &[
    Input::Hashed("7b12cf62b87f569224aa45eba659436d352cba8d7355d023044be5adf21cf099"),
    Input::Inline(
        "example",
        str_block! {"
            0123
            1234
            8765
            9876
        "},
        Some(1),
        None,
    ),
    Input::Inline(
        "larger example",
        str_block! {"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "},
        Some(36),
        Some(81),
    ),
    Input::Inline(
        "fork",
        str_block! {"
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
        "},
        Some(2),
        None,
    ),
    Input::Inline(
        "four",
        str_block! {"
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....
        "},
        Some(4),
        Some(13),
    ),
    Input::Inline(
        "two trailheads",
        str_block! {"
            10..9..
            2...8..
            3...7..
            4567654
            ...8..3
            ...9..2
            .....01
        "},
        Some(3),
        None,
    ),
];

struct Map<'a> {
    map: &'a [u8],
    pitch: usize,
    width: i8,
    height: i8,
}

impl<'a> Map<'a> {
    pub fn new(input: &'a str) -> Self {
        let map = input.as_bytes();
        let width = map.iter().position(|&b| b == b'\n').unwrap() as i8;
        let pitch = width as usize + 1;
        let height = (map.len() / pitch) as i8;
        Self {
            map,
            pitch,
            width,
            height,
        }
    }

    #[inline(always)]
    pub fn in_range(&self, x: i8, y: i8) -> bool {
        (x as u8) < self.width as u8 && (y as u8) < self.height as u8
    }

    #[inline(always)]
    pub fn get(&self, x: i8, y: i8) -> Option<u8> {
        self.in_range(x, y).then(|| {
            *self
                .map
                .get(y as usize * self.pitch + x as usize)
                .unwrap_or_else(|| unsafe { unreachable_unchecked() })
        })
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, x: i8, y: i8) -> u8 {
        self.get(x, y)
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
    }
}

pub fn part1(input: &str) -> u32 {
    let map = Map::new(input);
    let mut queue = Vec::with_capacity(input.len());
    let mut found = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if unsafe { map.get_unchecked(x, y) } == b'9' {
                let mut found_set = [0_u64; 0x40];
                queue.push((x, y));
                while let Some((x, y)) = queue.pop() {
                    let tile = unsafe { map.get_unchecked(x, y) } - 1;
                    for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                        let (x, y) = (x + dx, y + dy);
                        if map.get(x, y) == Some(tile) {
                            if tile == b'0' {
                                let bit = 1 << x;
                                found += ((found_set[y as usize] & bit) == 0) as u32;
                                found_set[y as usize] |= bit;
                            } else {
                                queue.push((x, y));
                            }
                        }
                    }
                }
            }
        }
    }
    found
}

pub fn part2(input: &str) -> u32 {
    let map = Map::new(input);
    let mut memo = vec![-1_i32; map.width as usize * map.height as usize];
    let mut found = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if unsafe { map.get_unchecked(x, y) } == b'9' {
                fn get(map: &Map, memo: &mut [i32], x: i8, y: i8) -> u32 {
                    let mut found = 0;
                    let tile = unsafe { map.get_unchecked(x, y) } - 1;
                    for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
                        let (x, y) = (x + dx, y + dy);
                        if map.get(x, y) == Some(tile) {
                            let mi = y as usize * map.width as usize + x as usize;
                            let m = memo[mi];
                            if m >= 0 {
                                found += m as u32;
                            } else {
                                let f = if tile == b'0' {
                                    1
                                } else {
                                    get(map, memo, x, y)
                                };
                                memo[mi] = f as _;
                                found += f;
                            }
                        }
                    }
                    found
                }
                found += get(&map, &mut memo, x, y);
            }
        }
    }
    found
}
