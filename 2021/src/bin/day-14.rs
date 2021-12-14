use std::{fmt::Display, num::NonZeroU8, str::FromStr};

const INPUT: &str = include_str!("day-14.input");

struct Polymer(Vec<u8>);

impl Polymer {
    fn step(&mut self, rules: &Rules) {
        for i in (1..self.0.len()).rev() {
            if let Some(insert) = rules.lookup(self.0[i - 1], self.0[i]) {
                self.0.insert(i, insert.get());
            }
        }
    }

    fn frequencies(&self) -> usize {
        let mut freq = [(0_usize, 0_u8); 256];
        for &b in self.0.iter() {
            freq[b as usize].0 += 1;
            freq[b as usize].1 = b;
        }
        let mut freq = freq.into_iter().filter(|n| n.0 != 0).collect::<Vec<_>>();
        freq.sort_unstable();
        freq[freq.len() - 1].0 - freq[0].0
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        String::from_utf8_lossy(&self.0).fmt(f)
    }
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Vec::from_iter(s.trim().as_bytes().iter().copied())))
    }
}

struct Rules([Option<NonZeroU8>; 65536]);

impl Rules {
    fn lookup(&self, b0: u8, b1: u8) -> Option<NonZeroU8> {
        let pair = b0 as u16 + b1 as u16 * 256;
        self.0[pair as usize]
    }
}

impl FromStr for Rules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = [None; 65536];
        for line in s.lines() {
            let (pair, insert) = line.split_once(" -> ").unwrap();
            assert!(pair.len() == 2 && insert.len() == 1);
            let pair = pair.as_bytes();
            let pair = pair[0] as u16 + pair[1] as u16 * 256;
            let insert = NonZeroU8::new(insert.as_bytes()[0]).unwrap();
            map[pair as usize] = Some(insert);
        }
        Ok(Self(map))
    }
}

fn main() {
    let (polymer, rules) = INPUT.trim().split_once("\n\n").unwrap();
    let mut polymer = polymer.parse::<Polymer>().unwrap();
    let rules = rules.parse::<Rules>().unwrap();

    for _ in 0..10 {
        polymer.step(&rules);
    }

    println!("part 1: {}", polymer.frequencies());
}
