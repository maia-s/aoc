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
        let width = self.map[0].len();
        let height = self.map.len();
        let mut moved = true;
        while moved {
            moved = false;
            for y in 0..height - 1 {
                for x in 0..width {
                    if self.map[y][x] == b'.' && self.map[y + 1][x] == b'O' {
                        moved = true;
                        self.map[y][x] = b'O';
                        self.map[y + 1][x] = b'.';
                    }
                }
            }
        }
        Ok(self.map.iter().enumerate().map(
            |(y, row)| row.iter().filter_map(|&c| (c == b'O').then_some(height - y)).sum::<usize>()
        ).sum())
    }

    test day14_example(INPUT_EX, 136);
    test day14(INPUT,);
}
