use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("day-19.input");

fn main() {
    let mut inputs = INPUT.splitn(2, "\n\n");
    let mut rules = inputs.next().unwrap().parse().unwrap();
    let messages = inputs.next().unwrap();

    println!("part 1: {}", matches(&rules, messages));

    rules.0.insert(8, Rule::OptSeq(vec![vec![42], vec![42, 8]]));
    rules
        .0
        .insert(11, Rule::OptSeq(vec![vec![42, 31], vec![42, 11, 31]]));

    println!("part 2: {}", matches(&rules, messages));
}

fn matches(rules: &Rules, messages: &str) -> usize {
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
        self.rests(0, s).contains("")
    }

    fn rests<'s>(&self, index: usize, s: &'s str) -> HashSet<&'s str> {
        if s.is_empty() {
            return HashSet::new();
        }
        match self.0.get(&index).unwrap() {
            &Rule::Char(ch) => {
                if s.chars().next().unwrap() == ch {
                    let mut rest = HashSet::new();
                    rest.insert(&s[1..]);
                    rest
                } else {
                    HashSet::new()
                }
            }
            Rule::OptSeq(opt) => {
                fn rests_seq<'s>(rules: &Rules, seq: &[usize], s: &'s str) -> HashSet<&'s str> {
                    if seq.is_empty() {
                        let mut rest = HashSet::new();
                        rest.insert(s);
                        rest
                    } else if s.is_empty() {
                        HashSet::new()
                    } else {
                        rules
                            .rests(seq[0], s)
                            .into_iter()
                            .flat_map(|rest| rests_seq(rules, &seq[1..], rest))
                            .collect()
                    }
                }

                opt.iter().flat_map(|seq| rests_seq(self, seq, s)).collect()
            }
        }
    }
}
