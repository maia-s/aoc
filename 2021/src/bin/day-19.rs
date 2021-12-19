use std::{collections::HashSet, str::FromStr};

const INPUT: &str = "";

struct Scanner {
    id: u32,
    configuration: Option<Configuration>,
    beacons: HashSet<Point>,
}

impl Scanner {}

impl FromStr for Scanner {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let title = it.next().unwrap();
        let id = title
            .trim_start_matches("--- scanner ")
            .trim_end_matches(" ---")
            .parse()
            .unwrap();
        let beacons = it.map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            id,
            configuration: None,
            beacons,
        })
    }
}

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

struct Configuration {
    origin: Point,
    orientation: Orientation,
}

impl Configuration {}

fn main() {
    let mut scanners: Vec<Scanner> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();
}
