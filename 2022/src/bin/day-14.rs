const INPUT: &str = include_str!("day-14.txt");

#[cfg(test)]
const INPUT_EX: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

aoc_2022::aoc! {
    #[derive(Clone)]
    struct Day14 {
        map: Box<[[u8; 1000]; 500]>,
        bounds_x: (usize, usize),
        bounds_y: (usize, usize),
    }

    self(input) {
        let map = Box::new([[b'.'; 1000]; 500]);
        let bounds_x = (1000, 0);
        let bounds_y = (1000, 0);
        let mut day = Self { map, bounds_x, bounds_y };
        for line in input.lines() {
            let mut it = line.split(" -> ");
            let p0 = it.next().ok_or("missing points")?;
            let p0 = p0.split_once(',').ok_or("expected `,`")?;
            let mut p0 = (p0.0.parse()?, p0.1.parse()?);
            for p1 in it {
                let p1 = p1.split_once(',').ok_or("expected `,`")?;
                let p1 = (p1.0.parse()?, p1.1.parse()?);
                day.line(p0, p1);
                p0 = p1;
            }
        }
        Ok(day)
    }

    part1 usize {
        let mut day = self.clone();
        let mut grains = 0;
        while day.grain((500, 0)) {
            grains += 1;
        }
        Ok(grains)
    }

    part2 usize {
        let mut grains = 0;
        while self.grain2((500, 0)) {
            grains += 1;
        }
        Ok(grains)
    }

    input = INPUT;
    test day14_ex(INPUT_EX, 24, 93);
    test day14(INPUT, 672, 26831);
}

impl Day14 {
    fn line(&mut self, (x0, y0): (usize, usize), (x1, y1): (usize, usize)) {
        self.bounds_x.0 = self.bounds_x.0.min(x0.min(x1));
        self.bounds_x.1 = self.bounds_x.1.max(x0.max(x1));
        self.bounds_y.0 = self.bounds_y.0.min(y0.min(y1));
        self.bounds_y.1 = self.bounds_y.1.max(y0.max(y1));
        if x0 < x1 {
            assert_eq!(y0, y1);
            for x in x0..=x1 {
                self.map[y0][x] = b'#';
            }
        } else if x1 < x0 {
            assert_eq!(y0, y1);
            for x in x1..=x0 {
                self.map[y0][x] = b'#';
            }
        } else if y0 <= y1 {
            for y in y0..=y1 {
                self.map[y][x0] = b'#';
            }
        } else {
            for y in y1..=y0 {
                self.map[y][x0] = b'#';
            }
        }
    }

    fn grain(&mut self, (mut x, mut y): (usize, usize)) -> bool {
        assert_eq!(self.map[y][x], b'.');
        loop {
            if y + 1 > self.bounds_y.1 {
                return false;
            } else if self.map[y + 1][x] == b'.' {
                y += 1;
            } else if x - 1 < self.bounds_x.0 {
                return false;
            } else if self.map[y + 1][x - 1] == b'.' {
                x -= 1;
                y += 1;
            } else if x + 1 > self.bounds_x.1 {
                return false;
            } else if self.map[y + 1][x + 1] == b'.' {
                x += 1;
                y += 1;
            } else {
                self.map[y][x] = b'o';
                return true;
            }
        }
    }

    fn grain2(&mut self, (mut x, mut y): (usize, usize)) -> bool {
        loop {
            if self.map[y][x] != b'.' {
                return false;
            } else if y + 1 == self.bounds_y.1 + 2 {
                self.map[y][x] = b'o';
                return true;
            } else if self.map[y + 1][x] == b'.' {
                y += 1;
            } else if self.map[y + 1][x - 1] == b'.' {
                x -= 1;
                y += 1;
            } else if self.map[y + 1][x + 1] == b'.' {
                x += 1;
                y += 1;
            } else {
                self.map[y][x] = b'o';
                return true;
            }
        }
    }
}
