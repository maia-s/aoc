use std::{num::NonZeroU8, str::FromStr};

const INPUT: &str = include_str!("day-14.input");

struct Polymer(Vec<u8>);

impl Polymer {
    fn run(&mut self, rules: &Rules, n: usize) -> usize {
        let mut freq = [(0_usize, 0_u8); 256];

        for i in 0..=255 {
            freq[i as usize].1 = i;
        }

        fn step(freq: &mut [(usize, u8)], rules: &Rules, b0: u8, b1: u8, depth: usize) {
            if depth != 0 {
                if let Some(insert) = rules.lookup(b0, b1) {
                    let insert = insert.get();
                    freq[insert as usize].0 += 1;
                    step(freq, rules, b0, insert, depth - 1);
                    step(freq, rules, insert, b1, depth - 1);
                }
            }
        }

        freq[self.0[0] as usize].0 += 1;

        for b in self.0.windows(2) {
            eprint!(".");
            freq[b[1] as usize].0 += 1;
            step(&mut freq, rules, b[0], b[1], n);
        }
        eprintln!();

        let mut freq = freq.into_iter().filter(|n| n.0 != 0).collect::<Vec<_>>();
        freq.sort_unstable();
        freq[freq.len() - 1].0 - freq[0].0
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

    println!("part 1: {}", polymer.run(&rules, 10));
    println!("part 2: {}", polymer.run(&rules, 40));
}
