use std::{error::Error, num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("day-1.txt");

fn main() -> Result<(), Box<dyn Error>> {
    let mut elves = Vec::<Elf>::new();
    for elf in INPUT.trim().split("\n\n") {
        elves.push(elf.parse()?);
    }

    elves.sort_unstable_by(|a, b| b.cmp(a));

    println!("part 1: {}", elves[0].calories);
    println!("part 2: {}", elves[0].calories + elves[1].calories + elves[2].calories);
    Ok(())
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
