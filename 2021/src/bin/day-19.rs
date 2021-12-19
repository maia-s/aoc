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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
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

impl Configuration {
    fn map_point(&self, dst: &Configuration, point: Point) -> Point {
        todo!()
    }
}

enum Orientation {}

fn main() {
    let mut scanners: Vec<Scanner> = INPUT.split("\n\n").map(|s| s.parse().unwrap()).collect();
}
