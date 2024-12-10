use crate::Input;
use rustc_hash::FxHashSet;
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
        None,
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
        None,
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
    queue: Vec<(u16, i8, i8)>,
    width: u8,
    height: u8,
}

impl<'a> Map<'a> {
    pub fn parse(input: &'a str) -> Self {
        let map = input.as_bytes();
        let width = map.iter().position(|&b| b == b'\n').unwrap() as u8;
        let pitch = width as usize + 1;
        let height = (map.len() / pitch) as u8;
        let mut queue = Vec::with_capacity(input.len());
        let mut id = 0;
        for y in 0..height {
            for x in 0..width {
                if map[y as usize * pitch + x as usize] == b'0' {
                    queue.push((id, x as i8, y as i8));
                    id += 1;
                }
            }
        }
        Self {
            map,
            queue,
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
    let mut map = Map::parse(input);
    let mut found = FxHashSet::default();
    while let Some((id, x, y)) = map.queue.pop() {
        let tile = map.get(x, y).unwrap() + 1;
        for (x, y) in [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)] {
            if map.get(x, y) == Some(tile) {
                if tile == b'9' {
                    found.insert((id, x, y));
                } else {
                    map.queue.push((id, x, y));
                }
            }
        }
    }
    found.len() as _
}

pub fn part2(input: &str) -> u32 {
    let mut map = Map::parse(input);
    let mut found = 0;
    while let Some((id, x, y)) = map.queue.pop() {
        let tile = map.get(x, y).unwrap() + 1;
        for (x, y) in [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)] {
            if map.get(x, y) == Some(tile) {
                if tile == b'9' {
                    found += 1;
                } else {
                    map.queue.push((id, x, y));
                }
            }
        }
    }
    found
}
