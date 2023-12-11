use aoc_2023::{aoc, str_block};

const INPUT: &str = include_str!("day-11.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"};

aoc! {
    struct Day11 {
        galaxies: Vec<(usize, usize)>,
        exp_rows: Vec<usize>,
        exp_cols: Vec<usize>,
    }

    self(input = INPUT) {
        let mut galaxies = Vec::new();
        let mut exp_rows = Vec::new();
        let map: Vec<&[u8]> = input.lines().enumerate().map(
            |(y, line)| {
                let line = line.as_bytes();
                if line.iter().all(|&c| c == b'.') {
                    exp_rows.push(y);
                }
                for x in line.iter().enumerate().filter(|(_, &b)| b == b'#').map(|(x, _)| x) {
                    galaxies.push((x, y));
                }
                line
            }
        ).collect();
        let exp_cols = (0..map[0].len()).filter(|&col|
            map.iter().map(|row| row[col]).all(|c| c == b'.')
        ).collect();
        Ok(Self {
            galaxies,
            exp_rows,
            exp_cols,
        })
    }

    part1 usize {
        Ok(self.dist_all(1))
    }

    part2 usize {
        Ok(self.dist_all(999_999))
    }

    test day11_example(INPUT_EX, 374);
    test day11(INPUT, 9536038, 447744640566);
}

impl Day11 {
    fn dist_all(&self, exp: usize) -> usize {
        self.galaxies[..self.galaxies.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, &ga)| {
                self.galaxies[i + 1..]
                    .iter()
                    .map(|&gb| self.dist(ga, gb, exp))
                    .sum::<usize>()
            })
            .sum()
    }

    fn dist(&self, (ax, ay): (usize, usize), (bx, by): (usize, usize), exp: usize) -> usize {
        self.dist_part(ax, bx, &self.exp_cols, exp) + self.dist_part(ay, by, &self.exp_rows, exp)
    }

    fn dist_part(&self, a: usize, b: usize, exps: &[usize], exp: usize) -> usize {
        let (a, b) = (a.min(b), a.max(b));
        let mut d = b - a;
        for i in exps.iter() {
            if (a..b).contains(i) {
                d += exp;
            }
        }
        d
    }
}
