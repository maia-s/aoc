// #![feature(min_const_generics)]

use std::{
    collections::HashSet,
    ops::{Add, Index, IndexMut, RangeBounds, Sub},
    str::FromStr,
};

const INPUT: &str = include_str!("day-17.input");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> usize {
    let mut cubes: Cubes<3> = INPUT.parse().unwrap();
    cubes.step_n(2..=3, 3, 6);
    cubes.len()
}

fn part_2() -> usize {
    let mut cubes: Cubes<4> = INPUT.parse().unwrap();
    cubes.step_n(2..=3, 3, 6);
    cubes.len()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coord<const N: usize>([isize; N]);

impl From<[isize; 2]> for Coord<3> {
    fn from(v: [isize; 2]) -> Self {
        Self([v[0], v[1], 0])
    }
}

impl From<[isize; 2]> for Coord<4> {
    fn from(v: [isize; 2]) -> Self {
        Self([v[0], v[1], 0, 0])
    }
}

impl From<[isize; 3]> for Coord<3> {
    fn from(v: [isize; 3]) -> Self {
        Self(v)
    }
}

impl From<[isize; 4]> for Coord<4> {
    fn from(v: [isize; 4]) -> Self {
        Self(v)
    }
}

impl<const N: usize> Index<usize> for Coord<N> {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Coord<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Coord<N> {
    const fn new(value: isize) -> Self {
        Self([value; N])
    }

    const fn len(&self) -> usize {
        self.0.len()
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|&c| c == 0)
    }

    #[inline]
    fn map2(mut self, other: Self, f: impl Fn(isize, isize) -> isize) -> Self {
        for (c, o) in self.0.iter_mut().zip(other.0.iter()) {
            *c = f(*c, *o);
        }
        self
    }

    fn min(self, other: Self) -> Self {
        self.map2(other, |l, r| l.min(r))
    }

    fn max(self, other: Self) -> Self {
        self.map2(other, |l, r| l.max(r))
    }

    fn iter(self, end: Self) -> CoordIter<N> {
        let mut current = self;
        current[0] -= 1;
        CoordIter {
            start: self,
            end,
            current,
        }
    }
}

impl<const N: usize> Add for Coord<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.map2(rhs, |l, r| l + r)
    }
}

impl<const N: usize> Add<isize> for Coord<N> {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        self + Self::new(rhs)
    }
}

impl<const N: usize> Sub<isize> for Coord<N> {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self::Output {
        self.map2(Self::new(rhs), |l, r| l - r)
    }
}

#[derive(Debug)]
struct CoordIter<const N: usize> {
    start: Coord<N>,
    end: Coord<N>,
    current: Coord<N>,
}

impl<const N: usize> Iterator for CoordIter<N> {
    type Item = Coord<N>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.current.len() {
            self.current[i] += 1;
            if self.current[i] <= self.end[i] {
                let item = self.current;
                return Some(item);
            } else {
                self.current[i] = self.start[i];
            }
        }
        None
    }
}

struct Cubes<const N: usize> {
    set: HashSet<Coord<N>>,
    min: Coord<N>,
    max: Coord<N>,
}

// this conflicts with itself if it's generic
impl FromStr for Cubes<3> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Self {
            set: HashSet::new(),
            min: Coord::new(isize::MAX),
            max: Coord::new(isize::MIN),
        };
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cubes.set([x as isize, y as isize].into());
                }
            }
        }
        Ok(cubes)
    }
}

impl FromStr for Cubes<4> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cubes = Self {
            set: HashSet::new(),
            min: Coord::new(isize::MAX),
            max: Coord::new(isize::MIN),
        };
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cubes.set([x as isize, y as isize].into());
                }
            }
        }
        Ok(cubes)
    }
}

impl<const N: usize> Cubes<N> {
    fn len(&self) -> usize {
        self.set.len()
    }

    fn get(&self, coord: Coord<N>) -> bool {
        self.set.get(&coord).is_some()
    }

    fn set(&mut self, coord: Coord<N>) {
        if self.set.insert(coord) {
            self.update_bounds(coord);
        }
    }

    fn unset(&mut self, coord: Coord<N>) {
        self.set.remove(&coord);
    }

    fn neighbors(&self, coord: Coord<N>) -> u32 {
        Coord::new(-1)
            .iter(Coord::new(1))
            .filter_map(|dc| {
                if !dc.is_zero() {
                    Some(self.get(coord + dc) as u32)
                } else {
                    None
                }
            })
            .sum()
    }

    fn reset_bounds(&mut self) {
        self.min = Coord::new(isize::MAX);
        self.max = Coord::new(isize::MIN);
    }

    fn update_bounds(&mut self, coord: Coord<N>) {
        self.min = self.min.min(coord);
        self.max = self.max.max(coord);
    }

    fn step(&mut self, stay: impl RangeBounds<u32>, spawn: u32) {
        let it = (self.min - 1).iter(self.max + 1);
        self.reset_bounds();
        let mut changes = vec![];

        for coord in it {
            if self.get(coord) {
                if stay.contains(&self.neighbors(coord)) {
                    self.update_bounds(coord);
                } else {
                    changes.push((false, coord));
                }
            } else {
                if self.neighbors(coord) == spawn {
                    changes.push((true, coord));
                }
            }
        }

        for (set, coord) in changes {
            if set {
                self.set(coord);
            } else {
                self.unset(coord);
            }
        }
    }

    fn step_n(&mut self, stay: impl Clone + RangeBounds<u32>, spawn: u32, n: usize) {
        for _ in 0..n {
            self.step(stay.clone(), spawn);
        }
    }
}
