use std::{
    collections::{HashSet, VecDeque},
    ops::{Add, Sub},
    str::FromStr,
};

const INPUT: &str = include_str!("day-19.input");

struct Scanner {
    beacons: HashSet<Point>,
}

impl Scanner {
    fn len(&self) -> usize {
        self.beacons.len()
    }

    fn combine(&mut self, other: &Self) -> bool {
        const OVERLAP: usize = 12;
        let slen = self.len();
        let olen = other.len();
        for &scb in self.beacons.iter() {
            for &ocb in other.beacons.iter() {
                let translate = ocb - scb;
                for o in ALL_ORIENTATIONS {
                    let mut merged = self.beacons.clone();
                    for &ob in other.beacons.iter() {
                        merged.insert((ob - translate).rotate_with_origin(scb, o));
                    }
                    let mlen = merged.len();
                    if mlen + OVERLAP <= slen + olen {
                        self.beacons = merged;
                        return true;
                    }
                }
            }
        }
        false
    }
}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let _title = it.next().unwrap();
        /*
        let id = title
            .trim_start_matches("--- scanner ")
            .trim_end_matches(" ---")
            .parse()
            .unwrap();
        */
        let beacons = it.map(|s| s.parse().unwrap()).collect();
        Ok(Self { beacons })
    }
}

const ALL_ORIENTATIONS: [Orientation; 24] = [
    Orientation::X0Y0Z0,
    Orientation::X0Y0Z1,
    Orientation::X0Y0Z2,
    Orientation::X0Y0Z3,
    Orientation::X0Y1Z0,
    Orientation::X0Y1Z1,
    Orientation::X0Y1Z2,
    Orientation::X0Y1Z3,
    Orientation::X0Y2Z0,
    Orientation::X0Y2Z1,
    Orientation::X0Y2Z2,
    Orientation::X0Y2Z3,
    Orientation::X0Y3Z0,
    Orientation::X0Y3Z1,
    Orientation::X0Y3Z2,
    Orientation::X0Y3Z3,
    Orientation::X1Y0Z0,
    Orientation::X1Y0Z1,
    Orientation::X1Y0Z2,
    Orientation::X1Y0Z3,
    Orientation::X3Y0Z0,
    Orientation::X3Y0Z1,
    Orientation::X3Y0Z2,
    Orientation::X3Y0Z3,
];

#[derive(Clone, Copy)]
enum Orientation {
    X0Y0Z0,
    X0Y0Z1,
    X0Y0Z2,
    X0Y0Z3,

    X0Y1Z0,
    X0Y1Z1,
    X0Y1Z2,
    X0Y1Z3,

    X0Y2Z0,
    X0Y2Z1,
    X0Y2Z2,
    X0Y2Z3,

    X0Y3Z0,
    X0Y3Z1,
    X0Y3Z2,
    X0Y3Z3,

    X1Y0Z0,
    X1Y0Z1,
    X1Y0Z2,
    X1Y0Z3,

    X3Y0Z0,
    X3Y0Z1,
    X3Y0Z2,
    X3Y0Z3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    #[must_use]
    fn rotate_with_origin(self, origin: Point, o: Orientation) -> Point {
        (self - origin).rotate(o) + origin
    }

    #[must_use]
    fn rotate(&self, o: Orientation) -> Point {
        match o {
            Orientation::X0Y0Z0 => *self,
            Orientation::X0Y0Z1 => Point {
                x: self.y,
                y: -self.x,
                z: self.z,
            },
            Orientation::X0Y0Z2 => Point {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },
            Orientation::X0Y0Z3 => Point {
                x: -self.y,
                y: self.x,
                z: self.z,
            },

            Orientation::X0Y1Z0 => Point {
                x: -self.z,
                y: self.y,
                z: self.x,
            },
            Orientation::X0Y1Z1 => Point {
                x: self.y,
                y: self.z,
                z: self.x,
            },
            Orientation::X0Y1Z2 => Point {
                x: self.z,
                y: -self.y,
                z: self.x,
            },
            Orientation::X0Y1Z3 => Point {
                x: -self.y,
                y: -self.z,
                z: self.x,
            },

            Orientation::X0Y2Z0 => Point {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            Orientation::X0Y2Z1 => Point {
                x: self.y,
                y: self.x,
                z: -self.z,
            },
            Orientation::X0Y2Z2 => Point {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            Orientation::X0Y2Z3 => Point {
                x: -self.y,
                y: -self.x,
                z: -self.z,
            },

            Orientation::X0Y3Z0 => Point {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            Orientation::X0Y3Z1 => Point {
                x: self.y,
                y: -self.z,
                z: -self.x,
            },
            Orientation::X0Y3Z2 => Point {
                x: -self.z,
                y: -self.y,
                z: -self.x,
            },
            Orientation::X0Y3Z3 => Point {
                x: -self.y,
                y: self.z,
                z: -self.x,
            },

            Orientation::X1Y0Z0 => Point {
                x: self.x,
                y: -self.z,
                z: self.y,
            },
            Orientation::X1Y0Z1 => Point {
                x: -self.z,
                y: -self.x,
                z: self.y,
            },
            Orientation::X1Y0Z2 => Point {
                x: -self.x,
                y: self.z,
                z: self.y,
            },
            Orientation::X1Y0Z3 => Point {
                x: self.z,
                y: self.x,
                z: self.y,
            },

            Orientation::X3Y0Z0 => Point {
                x: self.x,
                y: self.z,
                z: -self.y,
            },
            Orientation::X3Y0Z1 => Point {
                x: -self.z,
                y: self.x,
                z: -self.y,
            },
            Orientation::X3Y0Z2 => Point {
                x: -self.x,
                y: -self.z,
                z: -self.y,
            },
            Orientation::X3Y0Z3 => Point {
                x: self.z,
                y: -self.x,
                z: -self.y,
            },
        }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.trim().split(',');
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        let z = it.next().unwrap().parse().unwrap();
        Ok(Self { x, y, z })
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

fn main() {
    let mut scanners: VecDeque<Scanner> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let mut combined = scanners.pop_front().unwrap();
    while let Some(next) = scanners.pop_front() {
        eprint!(" {}  \r", scanners.len());
        if !combined.combine(&next) {
            scanners.push_back(next);
        }
    }
    println!("part 1: {}", combined.len());
}
