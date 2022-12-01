use std::{error::Error, num::ParseIntError, str::FromStr};

use aoc_2022::AoC;

const INPUT: &str = include_str!("day-1.txt");

fn main() -> Result<(), Box<dyn Error>> {
    Day1::run(INPUT, None, None)
}

struct Day1 {
    elves: Vec<Elf>,
}

impl AoC for Day1 {
    fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut elves = Vec::<Elf>::new();
        for elf in INPUT.trim().split("\n\n") {
            elves.push(elf.parse()?);
        }
        elves.sort_unstable_by(|a, b| b.cmp(a));
        Ok(Self { elves })
    }

    fn part_1(&self) -> usize {
        self.elves[0].calories
    }

    fn part_2(&self) -> usize {
        self.elves[0].calories + self.elves[1].calories + self.elves[2].calories
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Elf {
    calories: usize,
}

impl FromStr for Elf {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut calories = 0;
        for food in s.split('\n') {
            calories += food.parse::<usize>()?;
        }
        Ok(Self { calories })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day() {
        Day1::run(INPUT, Some(64929), Some(193697)).unwrap();
    }
}
