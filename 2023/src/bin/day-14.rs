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
        map: Vec<Vec<u8>>,
    }

    self(input = INPUT) {
        Ok(Self { map: input.lines().map(|line| line.as_bytes().to_owned()).collect() })
    }

    part1 usize {
        self.tilt_north();
        Ok(self.support_beam_weight())
    }

    part2 usize {
        for _ in 0..1_000_000_000 {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }
        Ok(self.support_beam_weight())
    }

    test day14_example(INPUT_EX, 136, 64);
    test day14(INPUT, 109755);
}

impl Day14 {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn tilt(
        &mut self,
        xr: impl Clone + Iterator<Item = usize>,
        yr: impl Clone + Iterator<Item = usize>,
        dx: isize,
        dy: isize,
    ) {
        let mut moved = true;
        while moved {
            moved = false;
            for y in yr.clone() {
                let y1 = (y as isize + dy) as usize;
                for x in xr.clone() {
                    let x1 = (x as isize + dx) as usize;
                    if self.map[y][x] == b'.' && self.map[y1][x1] == b'O' {
                        moved = true;
                        self.map[y][x] = b'O';
                        self.map[y1][x1] = b'.';
                    }
                }
            }
        }
    }

    fn tilt_north(&mut self) {
        self.tilt(0..self.width(), 0..self.height() - 1, 0, 1);
    }

    fn tilt_west(&mut self) {
        self.tilt(0..self.width() - 1, 0..self.height(), 1, 0);
    }

    fn tilt_south(&mut self) {
        self.tilt(0..self.width(), (1..self.height()).rev(), 0, -1);
    }

    fn tilt_east(&mut self) {
        self.tilt((1..self.width()).rev(), 0..self.height(), -1, 0);
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
