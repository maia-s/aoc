use core::hint::unreachable_unchecked;

use str_block::str_block;

use crate::Input;

pub const INPUTS: &[Input] = &[
    Input::Hashed("e8e19e262ef9e5612357123f69cbdbddf226c9677130ca7ec0dc9d54aec97e1c"),
    Input::Inline(
        "4x4",
        str_block! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "},
        Some(140),
        None,
    ),
    Input::Inline(
        "OXO",
        str_block! {"
            OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO
        "},
        Some(772),
        None,
    ),
    Input::Inline(
        "larger example",
        str_block! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "},
        Some(1930),
        None,
    ),
];

struct Map {
    map: Vec<u8>,
    width: u8,
    height: u8,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let input = input.as_bytes();
        let first = input.split(|&b| b == b'\n').next().unwrap();
        let width = first.len();
        let mut map = Vec::with_capacity(width * width);
        map.extend_from_slice(first);
        for line in input[width + 1..].chunks_exact(width + 1) {
            map.extend_from_slice(&line[..width]);
        }
        let height = map.len() / width;
        Self {
            map,
            width: width as u8,
            height: height as u8,
        }
    }

    #[inline(always)]
    pub const fn in_range(&self, x: u8, y: u8) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: u8, y: u8) -> Option<u8> {
        self.in_range(x, y)
            .then(|| unsafe { self.get_unchecked(x, y) })
    }

    pub unsafe fn get_unchecked(&self, x: u8, y: u8) -> u8 {
        *self
            .map
            .get(y as usize * self.width as usize + x as usize)
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
    }

    pub unsafe fn get_unchecked_mut(&mut self, x: u8, y: u8) -> &mut u8 {
        self.map
            .get_mut(y as usize * self.width as usize + x as usize)
            .unwrap_or_else(|| unsafe { unreachable_unchecked() })
    }

    pub fn flood(&mut self, x: u8, y: u8, ch: u8) -> (u32, u32) {
        let mut area = 1;
        let mut edges = 0;
        let cm = ch & 0x3f;
        unsafe { *self.get_unchecked_mut(x, y) = cm };
        let mut fill = |x, y| {
            if let Some(c) = self.get(x, y) {
                if c == ch {
                    let (fa, fe) = self.flood(x, y, ch);
                    area += fa;
                    edges += fe;
                }
                edges += (c & 0x3f != cm) as u32;
            } else {
                edges += 1
            }
        };
        fill(x, y - 1);
        fill(x - 1, y);
        fill(x + 1, y);
        fill(x, y + 1);
        (area, edges)
    }
}

pub fn part1(input: &str) -> u32 {
    let mut map = Map::new(input);
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let c = unsafe { map.get_unchecked(x, y) };
            if c & 0xc0 != 0 {
                let (area, edges) = map.flood(x, y, c);
                sum += area * edges;
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    0
}
