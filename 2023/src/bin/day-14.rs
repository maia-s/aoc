use std::collections::{hash_map::Entry, HashMap};

use aoc_2023::{aoc, str_block};

const INPUT: &str = include_str!("day-14.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"};

aoc! {
    struct Day14 {
        map: Map,
    }

    self(input = INPUT) {
        Ok(Self { map: Map { map: input.lines().map(|line| line.as_bytes().to_owned()).collect() } })
    }

    part1 usize {
        self.map.tilt_north();
        Ok(self.map.support_beam_weight())
    }

    part2 usize {
        let mut memo = HashMap::new();
        let mut i = 1_000_000_000;
        while i > 0 {
            i -= 1;
            self.map.tilt_north();
            self.map.tilt_west();
            self.map.tilt_south();
            self.map.tilt_east();
            match memo.entry(self.map.clone()) {
                Entry::Vacant(e) => { e.insert(i); }
                Entry::Occupied(e) => {
                    let diff = e.get() - i;
                    i -= i / diff * diff;
                },
            }
        }
        Ok(self.map.support_beam_weight())
    }

    test day14_example(INPUT_EX, 136, 64);
    test day14(INPUT, 109755, 90928);
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn tilt<const DX: isize, const DY: isize>(
        &mut self,
        xr: impl Clone + Iterator<Item = usize>,
        yr: impl Clone + Iterator<Item = usize>,
        go: impl Fn(usize, usize) -> bool,
    ) {
        for y in yr {
            for x in xr.clone() {
                if self.map[y][x] == b'O' {
                    self.map[y][x] = b'.';
                    let (mut xi, mut yi) = (x as isize, y as isize);
                    while go(xi as usize, yi as usize)
                        && self.map[(yi + DY) as usize][(xi + DX) as usize] == b'.'
                    {
                        xi += DX;
                        yi += DY;
                    }
                    self.map[yi as usize][xi as usize] = b'O';
                }
            }
        }
    }

    fn tilt_north(&mut self) {
        self.tilt::<0, -1>(0..self.width(), 0..self.height(), |_, y| y != 0);
    }

    fn tilt_west(&mut self) {
        self.tilt::<-1, 0>(0..self.width(), 0..self.height(), |x, _| x != 0);
    }

    fn tilt_south(&mut self) {
        let height = self.height();
        self.tilt::<0, 1>(0..self.width(), (0..self.height()).rev(), move |_, y| {
            y != height - 1
        });
    }

    fn tilt_east(&mut self) {
        let width = self.width();
        self.tilt::<1, 0>((0..self.width()).rev(), 0..self.height(), move |x, _| {
            x != width - 1
        });
    }

    fn support_beam_weight(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .filter_map(|&c| (c == b'O').then_some(self.height() - y))
                    .sum::<usize>()
            })
            .sum()
    }
}
