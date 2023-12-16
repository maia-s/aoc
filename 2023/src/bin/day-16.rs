use aoc_2023::{aoc, str_block};

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

    part1 usize {
        let mut bmap = self.map.clone();
        for row in bmap.iter_mut() {
            for c in row.iter_mut() {
                *c = 0;
            }
        }

        let mut beams = vec![((0, 0), Dir::E)];
        let mut energized = 0;

        while let Some(((x, y), dir)) = beams.pop() {
            if bmap[y][x] & dir.0 == 0 {
                if bmap[y][x] == 0 {
                    energized += 1;
                }
                bmap[y][x] |= dir.0;

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
                    b'|' => if matches!(dir, Dir::E | Dir::W) {
                        push(Dir::N);
                        push(Dir::S);
                    } else {
                        push(dir);
                    }
                    b'-' => if matches!(dir, Dir::N | Dir::S) {
                        push(Dir::E);
                        push(Dir::W);
                    } else {
                        push(dir);
                    }
                    _ => panic!("unknown tile"),
                }
            }
        }

        Ok(energized)
    }

    test day16_example(INPUT_EX, 46);
}

impl Day16 {
    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Dir(u8);

impl Dir {
    const N: Self = Self(0b1000);
    const E: Self = Self(0b0100);
    const S: Self = Self(0b0010);
    const W: Self = Self(0b0001);

    fn mov(&self, x: usize, y: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        let mut x = x as isize;
        let mut y = y as isize;
        let w = w as isize;
        let h = h as isize;
        match *self {
            Dir::N => y -= 1,
            Dir::E => x += 1,
            Dir::S => y += 1,
            Dir::W => x -= 1,
            _ => panic!("can't move multiple directions"),
        }
        if (0..w).contains(&x) && (0..h).contains(&y) {
            Some((x as usize, y as usize))
        } else {
            None
        }
    }
}
