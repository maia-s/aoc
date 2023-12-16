use std::fmt::Display;

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-13.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"};

aoc! {
    struct Day13 {
        maps: Vec<Map>,
    }

    self(input = INPUT) {
        Ok(Self {
            maps: input.split("\n\n").map(|map|
                Map(map.trim().lines().map(|line| line.as_bytes().to_owned()).collect())
            ).collect()
        })
    }

    part1 usize {
        Ok(self.maps.iter().map(Map::score).sum::<Result<_, _>>()?)
    }

    test day13_example(INPUT_EX, 405);
    test day13(INPUT,);
}

#[derive(Clone)]
struct Map(Vec<Vec<u8>>);

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for c in row.iter() {
                write!(f, "{}", char::from(*c))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn score(&self) -> Result<usize, Error> {
        self.find_horizontal_mirror()
            .or_else(|| self.find_vertical_mirror())
            .ok_or_else(|| format!("no reflections found in\n{}", *self).into())
    }

    fn find_horizontal_mirror(&self) -> Option<usize> {
        'find: for y in 1..self.height() {
            let mut ym = y as isize - 1;
            let mut yp = y;
            while ym >= 0 && yp < self.height() {
                if self.0[ym as usize] != self.0[yp] {
                    continue 'find;
                }
                ym -= 1;
                yp += 1;
            }
            return Some(y * 100);
        }
        None
    }

    fn find_vertical_mirror(&self) -> Option<usize> {
        'find: for x in 1..self.width() {
            let mut xm = x as isize - 1;
            let mut xp = x;
            while xm >= 0 && xp < self.width() {
                for row in self.0.iter() {
                    if row[xm as usize] != row[xp] {
                        continue 'find;
                    }
                }
                xm -= 1;
                xp += 1;
            }
            return Some(x);
        }
        None
    }
}
