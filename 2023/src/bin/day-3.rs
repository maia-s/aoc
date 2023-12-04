use std::{collections::HashMap, str::from_utf8};

use aoc_2023::str_block;

const INPUT: &str = include_str!("day-3.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."};

aoc_2023::aoc! {
    struct Day3 {
        sum: usize,
        gears: HashMap::<(usize, usize), Vec<usize>>,
    }

    self(input = INPUT) {
        let map: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
        let mut sum = 0;
        let mut gears: HashMap::<(usize, usize), Vec<usize>> = HashMap::new();
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
                    #[allow(clippy::needless_range_loop)]
                    for y in y0..=y1 {
                        for x in x0..=x1 {
                            if map[y][x] == b'*' {
                                gears.entry((x, y)).or_default().push(num);
                            }
                        }
                    }
                } else {
                    x += 1;
                }
            }
        }
        Ok(Self { sum, gears })
    }

    part1 usize {
        Ok(self.sum)
    }

    part2 usize {
        Ok(self.gears.values().filter(|gear| gear.len() == 2).map(|gear| gear[0] * gear[1]).sum())
    }

    test day3_example(INPUT_EX, 4361, 467835);
    test day3(INPUT, 509115, 75220503);
}
