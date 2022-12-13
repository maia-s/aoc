use std::{cmp::Ordering, error::Error};

const INPUT: &str = include_str!("day-13.txt");

#[cfg(test)]
const INPUT_EX: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

aoc_2022::aoc! {
    struct Day13 {
        pairs: Vec<(Packet, Packet)>,
    }

    self(input) {
        let mut pairs = Vec::new();
        for pair in input.split("\n\n") {
            let (l, r) = pair.split_once('\n').ok_or("expected pair")?;
            pairs.push((Packet::parse(l.trim())?.0, Packet::parse(r.trim())?.0));
        }
        Ok(Self { pairs })
    }

    part1 usize {
        Ok(self.pairs.iter().enumerate()
            .filter(|(_, pair)| pair.0.cmp(&pair.1) != Ordering::Greater)
            .map(|(i, _)| i+1)
            .sum())
    }

    part2 usize {
        let div2 = Packet::parse("[[2]]")?.0;
        let div6 = Packet::parse("[[6]]")?.0;
        let mut all: Vec<_> = self.pairs.iter().cloned().flat_map(|pair| [pair.0, pair.1]).collect();
        all.push(div2.clone());
        all.push(div6.clone());
        all.sort_unstable();
        let mut pos2 = 0;
        let mut pos6 = 0;
        for (i, p) in all.iter().enumerate() {
            if *p == div2 {
                pos2 = i + 1;
            } else if *p == div6 {
                pos6 = i + 1;
                break;
            }
        }
        Ok(pos2 * pos6)
    }

    input = INPUT;
    test day13_ex(INPUT_EX, 13, 140);
    test day13(INPUT, 5580, 26200);
}

#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Vec(Vec<Packet>),
    Int(usize),
}

impl Packet {
    fn parse(mut s: &str) -> Result<(Self, &str), Box<dyn Error>> {
        match s.chars().next() {
            Some('[') => {
                s = &s[1..];
                let mut v = Vec::new();
                if let Some(s) = s.strip_prefix(']') {
                    return Ok((Packet::Vec(v), s));
                }
                loop {
                    let (p, new_s) = Packet::parse(s)?;
                    s = new_s;
                    v.push(p);
                    if !s.starts_with(',') {
                        break;
                    }
                    s = &s[1..];
                }
                if !s.starts_with(']') {
                    return Err("expected `]`".into());
                }
                Ok((Packet::Vec(v), &s[1..]))
            }
            Some('0'..='9') => {
                let mut it = s.as_bytes().iter().copied();
                let mut i = 0;
                while let Some(b'0'..=b'9') = it.next() {
                    i += 1;
                }
                let n = s[0..i].parse()?;
                Ok((Packet::Int(n), &s[i..]))
            }
            _ => Err("invalid packet".into()),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Vec(l), Packet::Vec(r)) => {
                for (l, r) in l.iter().zip(r.iter()) {
                    let cmp = l.cmp(r);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                l.len().cmp(&r.len())
            }
            (Packet::Vec(_), Packet::Int(r)) => self.cmp(&Packet::Vec(vec![Packet::Int(*r)])),
            (Packet::Int(l), Packet::Vec(_)) => Packet::Vec(vec![Packet::Int(*l)]).cmp(other),
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
