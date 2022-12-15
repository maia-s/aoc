use std::{collections::HashSet, error::Error, str::FromStr};

const INPUT: &str = concat!("2000000\n", include_str!("day-15.txt"));

#[cfg(test)]
const INPUT_EX: &str = concat!(
    "10\n",
    "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
);

aoc_2022::aoc! {
    struct Day15 {
        sensors: HashSet<Sensor>,
        beacons: HashSet<(isize, isize)>,
        check_y: isize,
        min_x: isize,
        max_x: isize,
    }

    self(input) {
        let (check_y, sensors) = input.split_once('\n').ok_or("input error")?;
        let check_y = check_y.parse()?;
        let sensors: HashSet<Sensor> = sensors.lines().map(|x| x.parse()).collect::<Result<_,_>>()?;
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let beacons = sensors.iter().map(|s| {
            min_x = min_x.min(s.pos.0 - s.dist);
            max_x = max_x.max(s.pos.0 + s.dist);
            s.beacon
        }).collect();
        Ok(Self { sensors, beacons, check_y, min_x, max_x })
    }

    part1 usize {
        Ok((self.min_x..=self.max_x).into_iter().map(|x|
            (self.sensors.iter().any(|s| s.contains((x, self.check_y)))
                && !self.beacons.contains(&(x, self.check_y))
            ) as usize
        ).sum())
    }

    part2 isize {
        let limit = 0..=(if self.check_y == 10 { 20 } else { 4_000_000 });
        for s in self.sensors.iter() {
            if let Some((x, y)) = s.for_edge(|(x, y)| {
                limit.contains(&x) && limit.contains(&y) &&
                !self.sensors.iter().any(|s| s.contains((x, y)))
            }) {
                return Ok(x * 4_000_000 + y);
            }
        }
        Err("not found".into())
    }

    input = INPUT;
    test day15_ex(INPUT_EX, 26, 56000011);
    test day15(INPUT, 5461729, 10621647166538);
}

#[derive(PartialEq, Eq, Hash)]
struct Sensor {
    pos: (isize, isize),
    beacon: (isize, isize),
    dist: isize,
}

impl Sensor {
    fn contains(&self, (x, y): (isize, isize)) -> bool {
        (x - self.pos.0).abs() + (y - self.pos.1).abs() <= self.dist
    }

    fn for_edge(&self, f: impl Fn((isize, isize)) -> bool) -> Option<(isize, isize)> {
        let edge = self.dist + 1;
        for i in 0..edge {
            for p in [
                (self.pos.0 - i, self.pos.1 - edge + i),
                (self.pos.0 + i, self.pos.1 - edge + i),
                (self.pos.0 - i, self.pos.1 + edge - i),
                (self.pos.0 + i, self.pos.1 + edge - i),
            ] {
                if f(p) {
                    return Some(p);
                }
            }
        }
        None
    }
}

impl FromStr for Sensor {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Sensor at ").ok_or("invalid input")?;
        let (s, b) = s.split_once(": closest beacon is at ").ok_or("input")?;
        fn parse_coord(s: &str) -> Result<(isize, isize), Box<dyn Error>> {
            let (x, y) = s.split_once(", ").ok_or("expected `, `")?;
            let x = x.strip_prefix("x=").ok_or("expected `x=`")?;
            let y = y.strip_prefix("y=").ok_or("expected `y=`")?;
            Ok((x.parse()?, y.parse()?))
        }
        let pos = parse_coord(s)?;
        let beacon = parse_coord(b)?;
        let dist = (pos.0 - beacon.0).abs() + (pos.1 - beacon.1).abs();
        Ok(Self { pos, beacon, dist })
    }
}
