use std::{collections::HashMap, fmt::Debug, iter, str::FromStr};

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-19.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"};

aoc! {
    struct Day19 {
        rules: Rules,
        parts: Parts,
    }

    self(input = INPUT) {
        let (rules, parts) = input.split_once("\n\n").ok_or("invalid format")?;
        let rules: Rules = rules.parse()?;
        let parts = parts.parse()?;
        Ok(Self { rules, parts })
    }

    1 part1 usize {
        Ok(self.parts.iter().filter_map(|part| {
            self.rules.accepts(part).then_some(part.score())
        }).sum())
    }

    2 part2 usize {
        Ok(self.rules.accepted_ranges().iter().map(Ranges::combinations).sum())
    }

    INPUT_EX { 1 part1 = 19114, 2 part2 = 167409079868000 }
    INPUT { 1 part1 = 331208, 2 part2 = 121464316215623 }
}

#[derive(Clone)]
struct Parts(Vec<Part>);

impl FromStr for Parts {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(str::parse).collect::<Result<_, _>>()?))
    }
}

impl Parts {
    fn iter(&self) -> impl Iterator<Item = &Part> {
        self.0.iter()
    }
}

#[derive(Clone)]
struct Part([usize; 4]);

impl Part {
    fn score(&self) -> usize {
        self.0.iter().sum()
    }
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').ok_or("missing `{`")?;
        let s = s.strip_suffix('}').ok_or("missing `}`")?;
        let mut it = s.split(',');
        let x = it
            .next()
            .ok_or("missing `x`")?
            .strip_prefix("x=")
            .ok_or("expected `x=`")?
            .parse()
            .map_err(|_| "parse error")?;
        let m = it
            .next()
            .ok_or("missing `m`")?
            .strip_prefix("m=")
            .ok_or("expected `m=`")?
            .parse()
            .map_err(|_| "parse error")?;
        let a = it
            .next()
            .ok_or("missing `a`")?
            .strip_prefix("a=")
            .ok_or("expected `a=`")?
            .parse()
            .map_err(|_| "parse error")?;
        let s = it
            .next()
            .ok_or("missing `s`")?
            .strip_prefix("s=")
            .ok_or("expected `s=`")?
            .parse()
            .map_err(|_| "parse error")?;
        Ok(Self([x, m, a, s]))
    }
}

#[derive(Clone)]
struct Rules(HashMap<String, Rule>);

impl FromStr for Rules {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|s| s.parse().map(|rule: Rule| (rule.name.clone(), rule)))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Rules {
    fn accepts(&self, part: &Part) -> bool {
        let mut target = "in";
        while target != "A" && target != "R" {
            if let Some(rule) = self.0.get(target) {
                target = rule.eval(part);
            } else {
                panic!("missing rule {target}");
            }
        }
        target == "A"
    }

    fn accepted_ranges(&self) -> Vec<Ranges> {
        let mut queue = vec![("in", Ranges::new())];
        let mut accepted = Vec::new();
        while let Some((target, ranges)) = queue.pop() {
            for (target, ranges) in self.0.get(target).unwrap().eval_ranges(ranges) {
                if target == "A" {
                    accepted.push(ranges);
                } else if target != "R" {
                    queue.push((target, ranges));
                }
            }
        }
        accepted
    }
}

#[derive(Clone)]
struct Rule {
    name: String,
    steps: Vec<Step>,
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, steps) = s.split_once('{').ok_or("missing `{`")?;
        let steps = steps
            .strip_suffix('}')
            .ok_or("missing `}`")?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            name: name.to_owned(),
            steps,
        })
    }
}

impl Rule {
    fn eval(&self, part: &Part) -> &str {
        for step in self.steps.iter() {
            if let Some(target) = step.eval(part) {
                return target;
            }
        }
        unreachable!()
    }

    fn eval_ranges(&self, mut ranges: Ranges) -> impl Iterator<Item = (&str, Ranges)> {
        let mut steps = self.steps.iter();
        iter::from_fn(move || {
            if let Some(step) = steps.next() {
                let (a, b) = step.eval_ranges(ranges);
                if let Some(b) = b {
                    ranges = b;
                } else {
                    while steps.next().is_some() {}
                }
                if let Some((target, ranges)) = a {
                    return Some((target, ranges));
                }
            }
            None
        })
    }
}

#[derive(Clone)]
struct Step {
    cond: Cond,
    target: String,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((cond, target)) = s.split_once(':') {
            Ok(Self {
                cond: cond.parse()?,
                target: target.to_owned(),
            })
        } else {
            Ok(Self {
                cond: Cond::Always,
                target: s.to_owned(),
            })
        }
    }
}

impl Step {
    fn eval(&self, part: &Part) -> Option<&str> {
        self.cond.eval(part).then_some(&self.target)
    }

    fn eval_ranges(&self, ranges: Ranges) -> (Option<(&str, Ranges)>, Option<Ranges>) {
        let (a, b) = self.cond.eval_ranges(ranges);
        (a.map(|a| (self.target.as_str(), a)), b)
    }
}

#[derive(Clone, Copy)]
enum Cond {
    Always,
    Less(Cat, usize),
    Greater(Cat, usize),
}

impl FromStr for Cond {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let cat = it.next().ok_or("missing category")?.try_into()?;
        let op = it.next().ok_or("missing op")?;
        let n = it.as_str().parse().map_err(|_| "parse error")?;
        match op {
            '<' => Ok(Self::Less(cat, n)),
            '>' => Ok(Self::Greater(cat, n)),
            _ => Err("unknown op".into()),
        }
    }
}

impl Cond {
    fn eval(&self, part: &Part) -> bool {
        match *self {
            Self::Always => true,
            Self::Less(cat, value) => part.0[cat as usize] < value,
            Self::Greater(cat, value) => part.0[cat as usize] > value,
        }
    }

    fn eval_ranges(&self, ranges: Ranges) -> (Option<Ranges>, Option<Ranges>) {
        match *self {
            Self::Always => (Some(ranges), None),
            Self::Less(cat, value) => (ranges.lt(cat, value), ranges.ge(cat, value)),
            Self::Greater(cat, value) => (ranges.gt(cat, value), ranges.le(cat, value)),
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum Cat {
    Xtreme,
    Musical,
    Aerodynamic,
    Shiny,
}

impl TryFrom<char> for Cat {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'x' => Ok(Self::Xtreme),
            'm' => Ok(Self::Musical),
            'a' => Ok(Self::Aerodynamic),
            's' => Ok(Self::Shiny),
            _ => Err("unknown category".into()),
        }
    }
}

impl TryFrom<usize> for Cat {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Xtreme),
            1 => Ok(Self::Musical),
            2 => Ok(Self::Aerodynamic),
            3 => Ok(Self::Shiny),
            _ => Err("invalid category".into()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Ranges([(usize, usize); 4]);

impl Debug for Ranges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Ranges {
    fn new() -> Self {
        Self([(1, 4001); 4])
    }

    fn combinations(&self) -> usize {
        self.0.iter().map(|(from, to)| to - from).product()
    }

    fn is_valid(&self) -> bool {
        self.0.iter().all(|(from, to)| from < to)
    }

    #[must_use]
    fn lt(mut self, cat: Cat, value: usize) -> Option<Self> {
        self.0[cat as usize].1 = self.0[cat as usize].1.min(value);
        self.is_valid().then_some(self)
    }

    #[must_use]
    fn le(self, cat: Cat, value: usize) -> Option<Self> {
        self.lt(cat, value + 1)
    }

    #[must_use]
    fn gt(self, cat: Cat, value: usize) -> Option<Self> {
        self.ge(cat, value + 1)
    }

    #[must_use]
    fn ge(mut self, cat: Cat, value: usize) -> Option<Self> {
        self.0[cat as usize].0 = self.0[cat as usize].0.max(value);
        self.is_valid().then_some(self)
    }
}
