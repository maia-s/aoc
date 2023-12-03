use std::{collections::HashMap, str::from_utf8};

const INPUT: &str = include_str!("day-3.txt");

#[allow(dead_code)]
const INPUT_EX: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

aoc_2023::aoc! {
    struct Day3 { sum: usize, }

    self(input = INPUT) {
        let map: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
        let mut sum = 0;
        for y in 0..map.len() {
            let mut x = 0;
            let linelen = map[y].len();
            while x < linelen {
                if map[y][x].is_ascii_digit() {
                    let x0 = x;
                    while x < linelen && map[y][x].is_ascii_digit() {
                        x += 1;
                    }
                    let num: usize = from_utf8(&map[y][x0..x]).unwrap().parse().unwrap();
                    let (x0, x1) = (x0.saturating_sub(1), (linelen - 1).min(x));
                    let (y0, y1) = (y.saturating_sub(1), (map.len() - 1).min(y + 1));
                    let attached = |y: usize| map[y][x0..=x1].iter().any(|c| !matches!(c, b'0'..=b'9' | b'.'));
                    if attached(y0) || attached(y) || attached(y1) {
                        sum += num;
                    }
                } else {
                    x += 1;
                }
            }
        }
        Ok(Self { sum })
    }

    part1 usize {
        Ok(self.sum)
    }

    test day3_example(INPUT_EX, 4361);
}
