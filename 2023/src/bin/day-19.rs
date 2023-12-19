use std::{collections::HashMap, str::FromStr};

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
        let mut rules: Rules = rules.parse()?;
        let parts = parts.parse()?;
        rules.collapse();
        Ok(Self { rules, parts })
    }

    1 part1 usize {
        Ok(self.parts.iter().filter_map(|part| {
            self.rules.accepts(part).then_some(part.score())
        }).sum())
    }

    INPUT_EX { 1 part1 = 19114 }
    INPUT { 1 part1 = 331208 }
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

    fn collapse(&mut self) {
        let mut changed = true;
        while changed {
            let mut names: Vec<String> = self.0.keys().cloned().collect();
            for i in (0..names.len()).rev() {
                let rule = &names[i];
                if let Some(target) = self.0.get(rule).unwrap().collapse() {
                    self.replace_target(rule, &target.to_owned());
                } else {
                    names.remove(i);
                }
            }
            changed = !names.is_empty();
            for name in names.into_iter() {
                self.0.remove(&name);
            }
        }
    }

    fn replace_target(&mut self, src: &str, dst: &String) {
        for rule in self.0.values_mut() {
            for step in rule.steps.iter_mut() {
                if step.target == src {
                    step.target = dst.to_owned();
                }
            }
        }
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

    fn collapse(&self) -> Option<&str> {
        let mut it = self.steps.iter();
        let target = it.next().unwrap().target.as_str();
        for step in it {
            if step.target != target {
                return None;
            }
        }
        Some(target)
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
