use std::{fmt::Display, str::FromStr};

const INPUT: &str = include_str!("day-13.input");

struct Dots {
    map: Vec<bool>,
    pitch: usize,
    width: usize,
    height: usize,
}

impl Dots {
    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::AlongX(x) => {
                assert!(x <= self.width / 2);
                for i in 1..=x {
                    if x + i >= self.width {
                        continue;
                    }
                    let xl = x - i;
                    let xr = x + i;
                    for y in 0..self.height {
                        self.map[y * self.pitch + xl] |= self.map[y * self.pitch + xr];
                    }
                }
                self.width = x;
            }
            Fold::AlongY(y) => {
                assert!(y <= self.height / 2);
                for i in 1..=y {
                    if y + i >= self.height {
                        continue;
                    }
                    let yu = y - i;
                    let yd = y + i;
                    for x in 0..self.width {
                        self.map[yu * self.pitch + x] |= self.map[yd * self.pitch + x];
                    }
                }
                self.height = y;
            }
        }
    }

    fn count_dots(&self) -> usize {
        let mut dots = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                dots += self.map[y * self.pitch + x] as usize;
            }
        }
        dots
    }
}

impl FromStr for Dots {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let it = s.lines().map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        });
        let mut width = 0;
        let mut height = 0;

        for (x, y) in it.clone() {
            width = width.max(x + 1);
            height = height.max(y + 1);
        }

        let mut map = vec![false; width * height];

        for (x, y) in it {
            map[y * width + x] = true;
        }

        Ok(Self {
            map,
            pitch: width,
            width,
            height,
        })
    }
}

impl Display for Dots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.map[y * self.pitch + x] {
                        '#'
                    } else {
                        ' '
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl FromStr for Fold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (along, n) = s.split_once('=').unwrap();
        let n = n.trim().parse().unwrap();
        Ok(match along {
            "fold along x" => Fold::AlongX(n),
            "fold along y" => Fold::AlongY(n),
            _ => panic!("unrecongized fold `{}`", s),
        })
    }
}

fn main() {
    let (dots, folds) = INPUT.trim().split_once("\n\n").unwrap();
    let mut dots = dots.parse::<Dots>().unwrap();
    let folds: Vec<Fold> = folds.lines().map(|s| s.parse().unwrap()).collect();

    dots.fold(folds[0]);

    println!("part 1: {}", dots.count_dots());

    for &fold in folds.iter().skip(1) {
        dots.fold(fold);
    }

    println!("part 2:");
    println!("{}", dots);
}
