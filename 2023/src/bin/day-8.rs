use std::collections::HashMap;

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-8.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"};

#[allow(dead_code)]
const INPUT_EX2: &str = str_block! {"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"};

#[allow(dead_code)]
const INPUT_EX3: &str = str_block! {"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"};

aoc! {
    struct Day8<'a> {
        path: Vec<Dir>,
        map: HashMap<&'a str, [&'a str; 2]>,
    }

    self(input = INPUT) {
        let mut input = input.lines();

        let path = input.next().ok_or("missing directions")?;
        let path = path.chars().map(|c| match c {
            'L' => Ok(Dir::Left),
            'R' => Ok(Dir::Right),
            _ => Err(Error::from("unknown direction")),
        }).collect::<Result<_, _>>()?;

        input.next();

        let map = input.map(|line| {
            let (node, paths) = line.split_once(" = (").ok_or("missing ` = (`")?;
            let (left, right) = paths.split_once(", ").ok_or("missing `, `")?;
            let right = right.trim_end_matches(')');
            Ok((node, [left, right]))
        }).collect::<Result<_, Error>>()?;

        Ok(Self { path, map })
    }

    part1 usize {
        Ok(self.steps("AAA", "ZZZ", self.path.iter().copied().cycle())?)
    }

    part2 usize {
        let step = self.path.iter().copied().cycle();
        let mut nodes = self.map.keys().copied().filter(|key| key.ends_with('A'))
            .map(|start| Node::new(start, step.clone())).collect::<Vec<_>>();
        let mut steps = 0;
        'next: loop {
            for node in nodes.iter_mut() {
                let s = node.run(&self.map, |s, n| s >= steps && n.ends_with('Z'))?;
                if s > steps {
                    steps = s;
                    continue 'next;
                }
            }
            return Ok(steps)
        }
    }

    test day8_example(INPUT_EX, 2);
    test day8_example2(INPUT_EX2, 6);
    test day8_example3(INPUT_EX3,, 6);
    test day8(INPUT, 18113);
}

impl Day8<'_> {
    fn steps(
        &self,
        src: &str,
        dest: &str,
        mut step: impl Iterator<Item = Dir>,
    ) -> Result<usize, Error> {
        let mut node = src;
        let mut steps = 0;
        while node != dest {
            steps += 1;
            node = self
                .map
                .get(&node)
                .ok_or_else(|| format!("missing node `{node}`"))?[step.next().unwrap() as usize];
        }
        Ok(steps)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Left = 0,
    Right = 1,
}

struct Node<'a, I: Iterator<Item = Dir>> {
    current: &'a str,
    step: I,
    steps_taken: usize,
}

impl<'a, I: Iterator<Item = Dir>> Node<'a, I> {
    fn new(start: &'a str, step: I) -> Self {
        Self {
            current: start,
            step,
            steps_taken: 0,
        }
    }

    fn run(
        &mut self,
        map: &HashMap<&'a str, [&'a str; 2]>,
        stop: impl Fn(usize, &str) -> bool,
    ) -> Result<usize, Error> {
        while !stop(self.steps_taken, self.current) {
            self.steps_taken += 1;
            self.current = map
                .get(&self.current)
                .ok_or_else(|| format!("missing node {}", self.current))?
                [self.step.next().unwrap() as usize];
        }
        Ok(self.steps_taken)
    }
}
