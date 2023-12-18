use aoc_2023::{aoc, str_block, Dir, Error};

const INPUT: &str = include_str!("day-16.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"};

aoc! {
    struct Day16 {
        map: Vec<Vec<u8>>,
    }

    self(input = INPUT) {
        Ok(Self { map: input.lines().map(|line| line.as_bytes().to_owned()).collect() })
    }

    1 part1 usize {
        Ok(self.trace_beam(0, 0, Dir::E)?)
    }

    2 part2 usize {
        let mut max = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if x == 0 {
                    max = max.max(self.trace_beam(x, y, Dir::E)?);
                }
                if x == self.width() - 1 {
                    max = max.max(self.trace_beam(x, y, Dir::W)?);
                }
                if y == 0 {
                    max = max.max(self.trace_beam(x, y, Dir::S)?);
                }
                if y == self.height() - 1 {
                    max = max.max(self.trace_beam(x, y, Dir::N)?);
                }
            }
        }
        Ok(max)
    }

    INPUT_EX { 1 part1 = 46, 2 part2 = 51 }
    INPUT { 1 part1 = 7543, 2 part2 = 8231 }
}

impl Day16 {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn trace_beam(&self, x: usize, y: usize, dir: Dir) -> Result<usize, Error> {
        let mut bmap = self.map.clone();
        for row in bmap.iter_mut() {
            for c in row.iter_mut() {
                *c = 0;
            }
        }

        let mut beams = vec![((x, y), dir)];
        let mut energized = 0;

        while let Some(((x, y), dir)) = beams.pop() {
            if bmap[y][x] & dir.bits() == 0 {
                if bmap[y][x] == 0 {
                    energized += 1;
                }
                bmap[y][x] |= dir.bits();

                let mut push = |dir: Dir| {
                    if let Some((x, y)) = dir.mov(x, y, self.width(), self.height()) {
                        beams.push(((x, y), dir));
                    }
                };

                match self.map[y][x] {
                    b'.' => push(dir),
                    b'/' => push(match dir {
                        Dir::N => Dir::E,
                        Dir::E => Dir::N,
                        Dir::S => Dir::W,
                        Dir::W => Dir::S,
                        _ => unreachable!(),
                    }),
                    b'\\' => push(match dir {
                        Dir::N => Dir::W,
                        Dir::E => Dir::S,
                        Dir::S => Dir::E,
                        Dir::W => Dir::N,
                        _ => unreachable!(),
                    }),
                    b'|' => {
                        if matches!(dir, Dir::E | Dir::W) {
                            push(Dir::N);
                            push(Dir::S);
                        } else {
                            push(dir);
                        }
                    }
                    b'-' => {
                        if matches!(dir, Dir::N | Dir::S) {
                            push(Dir::E);
                            push(Dir::W);
                        } else {
                            push(dir);
                        }
                    }
                    _ => panic!("unknown tile"),
                }
            }
        }

        Ok(energized)
    }
}
