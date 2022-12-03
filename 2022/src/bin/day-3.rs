use std::{collections::HashSet, convert::Infallible, error::Error, str::FromStr};

const INPUT: &str = include_str!("day-3.txt");

#[cfg(test)]
const INPUT_EX: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

aoc_2022::aoc! {
    struct Day3 {
        rucksacks: Vec<Rucksack>,
    }

    self(input) {
        let mut rucksacks = Vec::new();
        for line in input.lines() {
            rucksacks.push(line.parse()?);
        }
        Ok(Self { rucksacks })
    }

    part1 {
        let mut priorities = 0;
        for rucksack in self.rucksacks.iter() {
            for i in rucksack.a.items.iter() {
                if rucksack.b.items.contains(i) {
                    priorities += priority(*i)?;
                }
            }
        }
        Ok(priorities)
    }

    part2 {
        let mut priorities = 0;
        for group in self.rucksacks.chunks(3) {
            let a = &group[0];
            let b = &group[1];
            let c = &group[2];
            for i in a.iter() {
                if b.contains(i) && c.contains(i) {
                    priorities += priority(i)?
                }
            }
        }
        Ok(priorities)
    }

    input = INPUT;
    test day3_ex(INPUT_EX, 157, 70);
    test day3(INPUT, 8109, 2738);
}

fn priority(i: u8) -> Result<usize, Box<dyn Error>> {
    Ok(match i {
        b'a'..=b'z' => i - b'a' + 1,
        b'A'..=b'Z' => i - b'A' + 27,
        _ => return Err("invalid item".into()),
    } as usize)
}

struct Rucksack {
    a: Compartment,
    b: Compartment,
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mid = s.len() / 2;
        let a = s[..mid].parse()?;
        let b = s[mid..].parse()?;
        Ok(Self { a, b })
    }
}

impl Rucksack {
    fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.a.items.iter().chain(self.b.items.iter()).copied()
    }

    fn contains(&self, i: u8) -> bool {
        self.a.items.contains(&i) || self.b.items.contains(&i)
    }
}

struct Compartment {
    items: HashSet<u8>,
}

impl FromStr for Compartment {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.as_bytes().iter().copied().collect();
        Ok(Self { items })
    }
}
