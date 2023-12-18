use std::{collections::HashSet, fmt::Debug, num::NonZeroU8, str::FromStr, usize};

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-12.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"};

aoc! {
    struct Day12 {
        records: Vec<Record>,
    }

    self(input = INPUT) {
        Ok(Self { records: input.lines().map(str::parse).collect::<Result<_, _>>()? })
    }

    1 part1 usize {
        Ok(self.records.iter_mut().map(Record::combinations).sum())
    }

    2 part2 usize {
        Ok(self.records.iter_mut().map(|r| {
            r.expand();
            r.combinations()
        }).sum())
    }

    INPUT_EX { 1 part1 = 21, 2 part2 = 525152 }
    INPUT { 1 part1 = 7599 }
}

#[derive(Clone)]
struct Record {
    pattern: Vec<(Spring, Bounds)>,
    groups: Vec<u8>,
    min_len: usize,
}

impl Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Record {{")?;
        writeln!(f, "  pattern: [")?;
        for p in self.pattern.iter() {
            writeln!(f, "    {:?} {:?}", p.0, p.1)?;
        }
        writeln!(f, "  ],")?;
        write!(f, "  groups: [")?;
        let mut first = true;
        for g in self.groups.iter() {
            if first {
                first = false;
            } else {
                write!(f, ",")?;
            }
            write!(f, " {}", g)?;
        }
        if !first {
            write!(f, " ")?;
        }
        writeln!(f, "],")?;
        writeln!(f, "}}")
    }
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, groups) = s.split_once(' ').ok_or("missing space")?;
        let pattern: Vec<_> = pattern
            .chars()
            .map(Spring::try_from)
            .map(|s| s.map(|s| (s, Bounds::new())))
            .collect::<Result<_, _>>()?;
        let groups = groups
            .split(',')
            .map(|s| s.parse().map_err(|_| "parse_error"))
            .collect::<Result<Vec<_>, _>>()?;
        let min_len = groups.iter().map(|&n| n as usize).sum::<usize>() + groups.len() - 1;
        if pattern.len() < min_len {
            return Err("group can't fit in pattern".into());
        }
        Ok(Self {
            pattern,
            groups,
            min_len,
        })
    }
}

impl Record {
    fn expand(&mut self) {
        let mut patexp = self.pattern.clone();
        patexp.insert(0, (Spring::Unknown, Bounds::new()));
        let groupexp = self.groups.clone();
        for _ in 0..4 {
            self.pattern.extend_from_slice(&patexp);
            self.groups.extend_from_slice(&groupexp);
        }
        self.min_len =
            self.groups.iter().map(|&n| n as usize).sum::<usize>() + self.groups.len() - 1;
    }

    fn annotate(&mut self) {
        for (_, bounds) in self.pattern.iter_mut() {
            bounds.clear();
        }
        for run in self
            .groups
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
        {
            'next: for i in 0..=self.pattern.len() - run as usize {
                if i > 0 && matches!(self.pattern[i - 1], (Spring::Damaged, _)) {
                    continue;
                }
                let mut j = i;
                while matches!(self.pattern.get(j), Some((Spring::Operational, _))) {
                    j += 1;
                }
                for j in j..=self.pattern.len() - run as usize {
                    if self.pattern[j..j + run as usize]
                        .iter()
                        .all(|(s, _)| *s != Spring::Operational)
                        && matches!(
                            self.pattern.get(j + run as usize),
                            Some((Spring::Operational | Spring::Unknown, _)) | None
                        )
                    {
                        self.pattern[i].1.set(run, (j - i).try_into().unwrap());
                        continue 'next;
                    }
                    if matches!(self.pattern[j], (Spring::Damaged, _)) {
                        continue 'next;
                    }
                }
            }
        }
    }

    fn combinations(&mut self) -> usize {
        fn rec(mut pattern: &[(Spring, Bounds)], mut groups: &[u8], mut min_len: usize) -> usize {
            let mut combs = 0;
            while let Some(skip) = pattern.get(0).and_then(|p| p.1.get(groups[0])) {
                if pattern.len() < min_len {
                    break;
                }
                if skip == 0 {
                    if matches!(pattern[0].0, Spring::Unknown) {
                        if let Some(skip2) = pattern.get(1).and_then(|p| p.1.get(groups[0])) {
                            combs += rec(&pattern[skip2 as usize + 1..], groups, min_len);
                        }
                    }
                    let mlen = groups[0] as usize + 1;
                    min_len = min_len.saturating_sub(mlen);
                    pattern = &pattern[mlen.min(pattern.len())..];
                    groups = &groups[1..];
                    if groups.is_empty() {
                        combs += pattern.iter().all(|p| !matches!(p.0, Spring::Damaged)) as usize;
                        break;
                    }
                } else {
                    pattern = &pattern[skip as usize..];
                }
            }
            combs
        }
        self.annotate();
        rec(&self.pattern, &self.groups, self.min_len)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Operational => '.',
                Self::Damaged => '#',
                Self::Unknown => '?',
            }
        )
    }
}

impl TryFrom<char> for Spring {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            x => Err(format!("invalid spring condition `{x}`").into()),
        }
    }
}

#[derive(Clone)]
struct Bounds(Vec<Option<NonZeroU8>>);

impl Debug for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        write!(f, "Bounds{{")?;
        for (run, dist) in self
            .0
            .iter()
            .enumerate()
            .filter(|(_, o)| o.is_some())
            .map(|(i, o)| (i + 1, !o.unwrap().get()))
        {
            if first {
                first = false
            } else {
                write!(f, ",")?;
            }
            write!(f, " {run} => {dist}")?;
        }
        if !first {
            write!(f, " ")?;
        }
        write!(f, "}}")
    }
}

impl Bounds {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    fn get(&self, len: u8) -> Option<u8> {
        Some(!(*self.0.get(len as usize - 1)?)?.get())
    }

    fn set(&mut self, len: u8, dist: u8) {
        let len = len as usize;
        if self.0.len() < len {
            self.0.resize(len, None);
        }
        self.0[len - 1] = NonZeroU8::new(!dist);
    }
}
