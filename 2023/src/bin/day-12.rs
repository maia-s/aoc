use std::{str::FromStr, usize};

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
        Ok(self.records.iter().map(Record::combinations).sum())
    }

    INPUT_EX { 1 part1 = 21 }
    INPUT { 1 part1 = 7599 }
}

#[derive(Clone)]
struct Record {
    pattern: Vec<Spring>,
    groups: Vec<usize>,
    min_len: usize,
}

impl FromStr for Record {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, groups) = s.split_once(' ').ok_or("missing space")?;
        let pattern: Vec<_> = pattern
            .chars()
            .map(Spring::try_from)
            .collect::<Result<_, _>>()?;
        let groups = groups
            .split(',')
            .map(|s| s.parse().map_err(|_| "parse_error"))
            .collect::<Result<Vec<_>, _>>()?;
        let min_len = groups.iter().sum::<usize>() + groups.len() - 1;
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
    fn combinations(&self) -> usize {
        fn rec(mut pattern: &[Spring], mut groups: &[usize], mut min_len: usize) -> usize {
            let mut combs = 0;
            'next: loop {
                if pattern.len() < min_len {
                    return combs;
                }
                let mut pat0 = pattern[0];
                loop {
                    match pat0 {
                        Spring::Operational => {
                            pattern = &pattern[1..];
                            continue 'next;
                        }

                        Spring::Damaged => {
                            let glen = groups[0];
                            if pattern[1..glen].iter().any(|&p| p == Spring::Operational)
                                || pattern.get(glen).copied() == Some(Spring::Damaged)
                            {
                                return combs;
                            }
                            if let Some(&s) = pattern.get(glen) {
                                if s == Spring::Damaged {
                                    return combs;
                                }
                            } else {
                                return combs + 1;
                            }
                            pattern = &pattern[glen + 1..];
                            groups = &groups[1..];
                            if groups.is_empty() {
                                if pattern.iter().any(|&p| p == Spring::Damaged) {
                                    return combs;
                                } else {
                                    return combs + 1;
                                }
                            }
                            min_len -= glen + 1;
                            continue 'next;
                        }

                        Spring::Unknown => {
                            combs += rec(&pattern[1..], groups, min_len);
                            pat0 = Spring::Damaged;
                            continue;
                        }
                    }
                }
            }
        }
        rec(&self.pattern, &self.groups, self.min_len)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
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
