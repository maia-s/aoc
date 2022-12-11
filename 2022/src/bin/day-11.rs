use std::{collections::VecDeque, error::Error, str::FromStr};

const INPUT: &str = include_str!("day-11.txt");

#[cfg(test)]
const INPUT_EX: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

aoc_2022::aoc! {
    #[derive(Clone)]
    struct Day11 {
        monkeys: Vec<Monkey>,
        modulo: usize,
    }

    self(input) {
        let monkeys: Vec<Monkey> = input.split("\n\n").map(|s| s.parse()).collect::<Result<_,_>>()?;
        let modulo = monkeys.iter().map(|m| m.test.test).product();
        Ok(Self { monkeys, modulo })
    }

    part1 usize {
        let mut day = self.clone();
        for _ in 0..20 {
            day.round(3);
        }
        Ok(day.monkey_biz())
    }

    part2 usize {
        for _ in 0..10_000 {
            self.round(1);
        }
        Ok(self.monkey_biz())
    }

    input = INPUT;
    test day11_ex(INPUT_EX, 10605, 2713310158);
    test day11(INPUT, 58056, 15048718170);
}

impl Day11 {
    fn round(&mut self, worry_div: usize) {
        let n = self.monkeys.len();
        for m in 0..n {
            while let Some((target, item)) = self.monkeys[m].inspect(worry_div, self.modulo) {
                self.monkeys[target].receive(item);
            }
        }
    }

    fn monkey_biz(&self) -> usize {
        let mut i: Vec<_> = self.monkeys.iter().map(|m| m.inspections).collect();
        i.sort_unstable_by(|a, b| b.cmp(a));
        i[0] * i[1]
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op: Op,
    test: Test,
    inspections: usize,
}

impl Monkey {
    fn inspect(&mut self, worry_div: usize, m: usize) -> Option<(usize, usize)> {
        if let Some(x) = self.items.pop_front() {
            let x = self.op.do_op(x) / worry_div % m;
            self.inspections += 1;
            let target = self.test.get_target_monkey(x);
            Some((target, x))
        } else {
            None
        }
    }

    fn receive(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (m, s) = s.split_once('\n').ok_or("missing line")?;
        let _ = m.strip_prefix("Monkey ").ok_or("expected monkey")?;
        let (items, s) = s.split_once('\n').ok_or("missing line")?;
        let items = items
            .trim()
            .strip_prefix("Starting items: ")
            .ok_or("expected items")?
            .split(", ")
            .map(|i| i.parse())
            .collect::<Result<_, _>>()?;
        let (op, s) = s.split_once('\n').ok_or("missing line")?;
        let op = op.parse()?;
        let test = s.parse()?;
        Ok(Monkey {
            items,
            op,
            test,
            inspections: 0,
        })
    }
}

#[derive(Clone, Copy)]
enum Op {
    OldPlusX(usize),
    OldTimesX(usize),
    OldTimesOld,
}

impl Op {
    fn do_op(&self, old: usize) -> usize {
        match self {
            Op::OldPlusX(x) => old + x,
            Op::OldTimesX(x) => old * x,
            Op::OldTimesOld => old * old,
        }
    }
}

impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("  Operation: new = old ")
            .ok_or("invalid operation")?;
        match &s[0..1] {
            "+" => {
                let s = s[1..].trim();
                Ok(Op::OldPlusX(s.parse()?))
            }
            "*" => {
                let s = s[1..].trim();
                if s == "old" {
                    Ok(Op::OldTimesOld)
                } else {
                    Ok(Op::OldTimesX(s.parse()?))
                }
            }
            _ => Err("unknown op".into()),
        }
    }
}

#[derive(Clone, Copy)]
struct Test {
    test: usize,
    if_true: usize,
    if_false: usize,
}

impl Test {
    fn get_target_monkey(&self, n: usize) -> usize {
        if n % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

impl FromStr for Test {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.lines();
        let test = s
            .next()
            .ok_or("expected line")?
            .strip_prefix("  Test: divisible by ")
            .ok_or("expected divisible by")?
            .parse()?;
        let if_true = s
            .next()
            .ok_or("expected line")?
            .strip_prefix("    If true: throw to monkey ")
            .ok_or("expected if true")?
            .parse()?;
        let if_false = s
            .next()
            .ok_or("expected line")?
            .strip_prefix("    If false: throw to monkey ")
            .ok_or("expected if false")?
            .parse()?;
        Ok(Self {
            test,
            if_true,
            if_false,
        })
    }
}
