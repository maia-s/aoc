use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("day-19.input");

fn main() {
    let mut inputs = INPUT.splitn(2, "\n\n");
    let rules = inputs.next().unwrap().parse().unwrap();
    let messages = inputs.next().unwrap();

    println!("part 1: {}", part_1(&rules, messages));
}

fn part_1(rules: &Rules, messages: &str) -> usize {
    messages
        .lines()
        .filter(|message| rules.matches(message))
        .count()
}

#[derive(Clone, Debug)]
enum Rule {
    Char(char),
    OptSeq(Vec<Vec<usize>>),
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            assert!(s.len() == 3 && s.ends_with('"'));
            Ok(Rule::Char(s.chars().nth(1).unwrap()))
        } else {
            Ok(Rule::OptSeq(
                s.split(" | ")
                    .map(|option| {
                        option
                            .split(' ')
                            .map(|rule| rule.parse().unwrap())
                            .collect()
                    })
                    .collect(),
            ))
        }
    }
}

#[derive(Clone, Debug)]
struct Rules(HashMap<usize, Rule>);

impl FromStr for Rules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        for line in s.lines() {
            let mut it = line.splitn(2, ": ");
            let index = it.next().unwrap().parse().unwrap();
            let rule = it.next().unwrap().parse().unwrap();
            rules.insert(index, rule);
        }
        Ok(Self(rules))
    }
}

impl Rules {
    fn matches<'s>(&self, s: &'s str) -> bool {
        if let Some(rests) = self._matches(0, s) {
            rests.len() == 1 && rests.iter().next().unwrap() == &""
        } else {
            false
        }
    }

    fn _matches<'s>(&self, index: usize, s: &'s str) -> Option<HashSet<&'s str>> {
        match self.0.get(&index).unwrap() {
            &Rule::Char(ch) => {
                if s.len() >= 1 && s.chars().next().unwrap() == ch {
                    let mut rest = HashSet::new();
                    rest.insert(&s[1..]);
                    Some(rest)
                } else {
                    None
                }
            }
            Rule::OptSeq(opt) => {
                fn match_seq<'s>(rules: &Rules, seq: &[usize], s: &'s str) -> HashSet<&'s str> {
                    if seq.is_empty() {
                        let mut rest = HashSet::new();
                        rest.insert(s);
                        rest
                    } else {
                        match rules._matches(seq[0], s) {
                            Some(v) => v
                                .into_iter()
                                .flat_map(|rest| match_seq(rules, &seq[1..], rest))
                                .collect(),
                            None => HashSet::new(),
                        }
                    }
                }

                let rests: HashSet<_> =
                    opt.iter().flat_map(|seq| match_seq(self, seq, s)).collect();
                if rests.is_empty() {
                    None
                } else {
                    Some(rests)
                }
            }
        }
    }
}
