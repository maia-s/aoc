use std::collections::VecDeque;

const INPUT: &str = include_str!("day-12.txt");

#[cfg(test)]
const INPUT_EX: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

aoc_2022::aoc! {
    struct Day12 {
        map: Vec<Vec<u8>>,
        start: (usize, usize),
        end: (usize, usize),
    }

    self(input) {
        let mut map: Vec<_> = input.lines().map(|line| line.trim().as_bytes().to_owned()).collect();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                match map[y][x] {
                    b'S' => {
                        map[y][x] = b'a';
                        start = (x, y);
                    }
                    b'E' => {
                        map[y][x] = b'z';
                        end = (x, y);
                    }
                    _ => (),
                }
            }
        }
        Ok(Self { map, start, end })
    }

    part1 usize {
        let mut map = self.map.clone();
        let mut steps = VecDeque::new();
        steps.push_back((0, self.start));
        while let Some((n, pos)) = steps.pop_front() {
            if pos == self.end {
                return Ok(n);
            }            
            let here = map[pos.1][pos.0];
            if here == b'|' {
                continue;
            }
            map[pos.1][pos.0] = b'|';
            if pos.0 > 0 && map[pos.1][pos.0 - 1] <= here + 1 {
                steps.push_back((n + 1, (pos.0 - 1, pos.1)));
            }
            if pos.0 < map[0].len() - 1 && map[pos.1][pos.0 + 1] <= here + 1 {
                steps.push_back((n + 1, (pos.0 + 1, pos.1)));
            }
            if pos.1 > 0 && map[pos.1 - 1][pos.0] <= here + 1 {
                steps.push_back((n + 1, (pos.0, pos.1 - 1)));
            }
            if pos.1 < map.len() - 1 && map[pos.1 + 1][pos.0] <= here + 1 {
                steps.push_back((n + 1, (pos.0, pos.1 + 1)));
            }
        }
        Err("no path".into())
    }

    part2 usize {
        let mut map = self.map.clone();
        let mut steps = VecDeque::new();
        steps.push_back((0, self.end));
        while let Some((n, pos)) = steps.pop_front() {
            let here = map[pos.1][pos.0];
            match here {
                b'|' => continue,
                b'a' => return Ok(n),
                _ => (),
            }
            map[pos.1][pos.0] = b'|';
            if pos.0 > 0 && map[pos.1][pos.0 - 1] >= here - 1 {
                steps.push_back((n + 1, (pos.0 - 1, pos.1)));
            }
            if pos.0 < map[0].len() - 1 && map[pos.1][pos.0 + 1] >= here - 1 {
                steps.push_back((n + 1, (pos.0 + 1, pos.1)));
            }
            if pos.1 > 0 && map[pos.1 - 1][pos.0] >= here - 1 {
                steps.push_back((n + 1, (pos.0, pos.1 - 1)));
            }
            if pos.1 < map.len() - 1 && map[pos.1 + 1][pos.0] >= here - 1 {
                steps.push_back((n + 1, (pos.0, pos.1 + 1)));
            }
        }
        Err("no path".into())
    }

    input = INPUT;
    test day12_ex(INPUT_EX, 31, 29);
    test day12(INPUT, 383, 377);
}
