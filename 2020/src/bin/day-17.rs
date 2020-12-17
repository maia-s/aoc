use std::{collections::HashSet, ops::RangeBounds, str::FromStr};

const INPUT: &str = include_str!("day-17.input");

fn main() {
    println!("part 1: {}", part_1());
}

fn part_1() -> usize {
    let mut cubes: Cubes = INPUT.parse().unwrap();
    cubes.step_n(2..=3, 3, 6);
    cubes.len()
}

type Coord = (isize, isize, isize);

#[derive(Default)]
struct Cubes {
    set: HashSet<Coord>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl FromStr for Cubes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Cubes::default();
        cubes.reset_bounds();
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cubes.set(x as isize, y as isize, 0);
                }
            }
        }
        Ok(cubes)
    }
}

impl Cubes {
    fn len(&self) -> usize {
        self.set.len()
    }

    fn get(&self, x: isize, y: isize, z: isize) -> bool {
        self.set.get(&(x, y, z)).is_some()
    }

    fn set(&mut self, x: isize, y: isize, z: isize) {
        if self.set.insert((x, y, z)) {
            self.update_bounds(x, y, z);
        }
    }

    fn unset(&mut self, x: isize, y: isize, z: isize) {
        self.set.remove(&(x, y, z));
    }

    fn neighbors(&self, x: isize, y: isize, z: isize) -> u32 {
        let mut total = 0;
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx != 0 || dy != 0 || dz != 0 {
                        let n = self.get(x + dx, y + dy, z + dz) as u32;
                        total += n;
                    }
                }
            }
        }
        total
    }

    fn reset_bounds(&mut self) {
        self.min_x = isize::MAX;
        self.min_y = isize::MAX;
        self.min_z = isize::MAX;
        self.max_x = isize::MIN;
        self.max_y = isize::MIN;
        self.max_z = isize::MIN;
    }

    fn update_bounds(&mut self, x: isize, y: isize, z: isize) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.min_z = self.min_z.min(z);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.max_z = self.max_z.max(z);
    }

    fn step(&mut self, stay: impl RangeBounds<u32>, spawn: u32) {
        let x_range = (self.min_x - 1)..=(self.max_x + 1);
        let y_range = (self.min_y - 1)..=(self.max_y + 1);
        let z_range = (self.min_z - 1)..=(self.max_z + 1);
        self.reset_bounds();
        let mut changes = vec![];

        for z in z_range {
            for y in y_range.clone() {
                for x in x_range.clone() {
                    if self.get(x, y, z) {
                        if stay.contains(&self.neighbors(x, y, z)) {
                            self.update_bounds(x, y, z);
                        } else {
                            changes.push((false, x, y, z));
                        }
                    } else {
                        if self.neighbors(x, y, z) == spawn {
                            changes.push((true, x, y, z));
                        }
                    }
                }
            }
        }

        for (set, x, y, z) in changes {
            if set {
                self.set(x, y, z);
            } else {
                self.unset(x, y, z);
            }
        }
    }

    fn step_n(&mut self, stay: impl Clone + RangeBounds<u32>, spawn: u32, n: usize) {
        for _ in 0..n {
            self.step(stay.clone(), spawn);
        }
    }
}
