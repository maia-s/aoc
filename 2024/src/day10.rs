use crate::Input;
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
    width: u8,
    height: u8,
}

impl<'a> Map<'a> {
    pub fn new(input: &'a str) -> Self {
        let map = input.as_bytes();
        let width = map.iter().position(|&b| b == b'\n').unwrap() as u8;
        let pitch = width as usize + 1;
        let height = (map.len() / pitch) as u8;
        Self {
            map,
            pitch,
            width,
            height,
        }
    }

    #[inline(always)]
    pub fn get(&self, x: i8, y: i8) -> Option<u8> {
        ((x as u8) < self.width && (y as u8) < self.height).then(|| unsafe {
            *self
                .map
                .get_unchecked(y as usize * (self.width as usize + 1) + x as usize)
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let map = Map::new(input);
    let mut queue = Vec::with_capacity(input.len());
    let mut found = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if unsafe { *map.map.get_unchecked(y as usize * map.pitch + x as usize) } == b'9' {
                let mut found_set = [0_u64; 0x40];
                queue.clear();
                queue.push((x as i8, y as i8));
                while let Some((x, y)) = queue.pop() {
                    let tile = map.get(x, y).unwrap() - 1;
                    for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
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
    let mut queue = Vec::with_capacity(input.len());
    let mut found = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if unsafe { *map.map.get_unchecked(y as usize * map.pitch + x as usize) } == b'9' {
                queue.push((x as i8, y as i8));
            }
        }
    }
    while let Some((x, y)) = queue.pop() {
        let tile = map.get(x, y).unwrap() - 1;
        for (dx, dy) in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
            let (x, y) = (x + dx, y + dy);
            if map.get(x, y) == Some(tile) {
                if tile == b'0' {
                    found += 1;
                } else {
                    queue.push((x, y));
                }
            }
        }
    }
    found
}
