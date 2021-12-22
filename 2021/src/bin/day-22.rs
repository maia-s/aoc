use std::{ops::RangeInclusive, str::FromStr};

const INPUT: &str = include_str!("day-22.input");

struct Reactor {
    steps: Vec<Step>,
    xr: RangeInclusive<isize>,
    yr: RangeInclusive<isize>,
    zr: RangeInclusive<isize>,
}

impl Reactor {
    fn get(&self, x: isize, y: isize, z: isize) -> bool {
        let mut on = false;
        for step in self.steps.iter() {
            if step.xr.contains(&x) && step.yr.contains(&y) && step.zr.contains(&z) {
                on = step.on;
            }
        }
        on
    }

    fn count(
        &self,
        xr: RangeInclusive<isize>,
        yr: RangeInclusive<isize>,
        zr: RangeInclusive<isize>,
    ) -> usize {
        let mut count = 0;
        let mut total = 0;
        let max =
            (xr.end() - xr.start() + 1) * (yr.end() - yr.start() + 1) * (zr.end() - zr.start() + 1);
        for z in zr {
            for y in yr.clone() {
                for x in xr.clone() {
                    count += self.get(x, y, z) as usize;
                    total += 1;
                }
                eprint!(" {}% \r", total * 100 / max);
            }
        }
        count
    }

    fn count_all(&self) -> usize {
        self.count(self.xr.clone(), self.yr.clone(), self.zr.clone())
    }
}

impl FromStr for Reactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps: Vec<Step> = s.lines().map(|line| line.parse().unwrap()).collect();
        let mut min_x = *steps[0].xr.start();
        let mut max_x = *steps[0].xr.end();
        let mut min_y = *steps[0].yr.start();
        let mut max_y = *steps[0].yr.end();
        let mut min_z = *steps[0].zr.start();
        let mut max_z = *steps[0].zr.end();
        for step in steps[1..].iter() {
            min_x = min_x.min(*step.xr.start());
            max_x = max_x.max(*step.xr.end());
            min_y = min_y.min(*step.yr.start());
            max_y = max_y.max(*step.yr.end());
            min_z = min_z.min(*step.zr.start());
            max_z = max_z.max(*step.zr.end());
        }
        Ok(Self {
            steps,
            xr: min_x..=max_x,
            yr: min_y..=max_y,
            zr: min_z..=max_z,
        })
    }
}

struct Step {
    on: bool,
    xr: RangeInclusive<isize>,
    yr: RangeInclusive<isize>,
    zr: RangeInclusive<isize>,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (on, range) = s.trim().split_once(' ').unwrap();
        let on = on == "on";
        let mut range = range.split(',');
        let xr = parse_range(range.next().unwrap());
        let yr = parse_range(range.next().unwrap());
        let zr = parse_range(range.next().unwrap());
        Ok(Self { on, xr, yr, zr })
    }
}

fn parse_range(s: &str) -> RangeInclusive<isize> {
    let (_, range) = s.split_once('=').unwrap();
    let (start, end) = range.split_once("..").unwrap();
    let start = start.parse().unwrap();
    let end = end.parse().unwrap();
    start..=end
}

fn main() {
    let reactor: Reactor = INPUT.parse().unwrap();

    let part_1 = reactor.count(-50..=50, -50..=50, -50..=50);
    println!("part 1: {}", part_1);

    let part_2 = reactor.count_all();
    println!("part 2: {}", part_2);
}
