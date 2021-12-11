use std::{str::FromStr, fmt::Debug};

const INPUT: &str = include_str!("day-11.input");

struct Octopuses {
    map: Vec<u8>,
    next: Vec<u8>,
    width: usize,
    flashes: usize,
}

impl Octopuses {
    fn step(&mut self) {
        fn step(map: &mut [u8], w: usize, x: usize, y: usize) -> usize {
            let mut flash = 0;
            if map[y * w + x] != 0xff {
                map[y * w + x] += 1;
                if map[y * w + x] > 9 {
                    map[y * w + x] = 0xff;
                    flash += 1;
                    if y > 0 {
                        if x > 0 {
                            flash += step(map, w, x - 1, y - 1);
                        }
                        flash += step(map, w, x, y - 1);
                        if x < w - 1 {
                            flash += step(map, w, x + 1, y - 1);
                        }
                    }
                    if x > 0 {
                        flash += step(map, w, x - 1, y);
                    }
                    if x < w - 1 {
                        flash += step(map, w, x + 1, y);
                    }
                    if y < map.len() / w - 1 {
                        if x > 0 {
                            flash += step(map, w, x - 1, y + 1);
                        }
                        flash += step(map, w, x, y + 1);
                        if x < w - 1 {
                            flash += step(map, w, x + 1, y + 1);
                        }
                    }
                }
            }
            flash
        }

        self.next.copy_from_slice(&self.map);

        for y in 0..self.map.len() / self.width {
            for x in 0..self.width {
                self.flashes += step(&mut self.next, self.width, x, y);
            }
        }

        for (&i, n) in self.next.iter().zip(self.map.iter_mut()) {
            *n = if i == 0xff { 0 } else { i };
        }
    }
}

impl FromStr for Octopuses {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let map: Vec<_> = s
            .trim()
            .lines()
            .flat_map(|line| {
                let line: Vec<_> = line.trim().chars().map(|c| c as u8 - b'0').collect();
                if width != 0 {
                    assert_eq!(width, line.len())
                } else {
                    width = line.len();
                }
                line
            })
            .collect();
        let n = map.len();
        Ok(Self {
            map,
            next: vec![0; n],
            width,
            flashes: 0,
        })
    }
}

impl Debug for Octopuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.len()/self.width {
            for x in 0..self.width {
                let i = self.map[y * self.width + x];
                write!(f, "{}", i)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut octopuses: Octopuses = INPUT.parse().unwrap();

    for _ in 0..100 {
        octopuses.step();
    }

    println!("part 1: {}", octopuses.flashes);
}
