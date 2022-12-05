use std::{num::ParseIntError, str::FromStr};

const INPUT: &str = include_str!("day-1.txt");

aoc_2022::aoc! {
    struct Day1 {
        elves: Vec<Elf>,
    }

    self(input) {
        let mut elves = Vec::<Elf>::new();
        for elf in input.trim().split("\n\n") {
            elves.push(elf.parse()?);
        }
        elves.sort_unstable_by(|a, b| b.cmp(a));
        Ok(Self { elves })
    }

    part1 usize {
        Ok(self.elves[0].calories)
    }

    part2 usize {
        Ok(self.elves[0].calories + self.elves[1].calories + self.elves[2].calories)
    }

    input = INPUT;
    test day1(INPUT, 64929, 193697);
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
