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
        let mut colacc = Vec::new();
        for (y, line) in input.lines().enumerate() {
            let line = line.as_bytes();
            if colacc.len() < line.len() {
                colacc.resize(line.len(), true);
            }
            let mut any_galaxies = false;
            for x in line.iter().enumerate().filter_map(|(x, &b)| {
                let has_galaxy = b == b'#';
                if has_galaxy {
                    colacc[x] = false;
                }
                has_galaxy.then_some(x)
            }) {
                any_galaxies = true;
                galaxies.push((x, y));
            }
            if !any_galaxies {
                exp_rows.push(y);
            }
        }
        let exp_cols = colacc.iter().enumerate().filter_map(|(x, c)| c.then_some(x)).collect();
        Ok(Self {
            galaxies,
            exp_rows,
            exp_cols,
        })
    }

    1 part1 usize {
        Ok(self.dist_all(1))
    }

    2 part2 usize {
        Ok(self.dist_all(999_999))
    }

    INPUT_EX { 1 part1 = 374 }
    INPUT { 1 part1 = 9536038, 2 part2 = 447744640566 }
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
