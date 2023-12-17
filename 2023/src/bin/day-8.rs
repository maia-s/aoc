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

    1 part1 usize {
        let mut step = self.path.iter().copied().cycle();
        let mut node = "AAA";
        let mut steps = 0;
        while node != "ZZZ" {
            steps += 1;
            node = self
                .map
                .get(&node)
                .ok_or_else(|| format!("missing node `{node}`"))?[step.next().unwrap() as usize];
        }
        Ok(steps)
    }

    2 part2 usize {
        // so not stated in the puzzle description but apparently something we can rely on
        // is that each start only has one possible end and repeats after that with the same
        // cycle length each time
        let step = self.path.iter().copied().cycle();
        Ok(
            self.map.keys().copied().filter(|key| key.ends_with('A'))
                .map(|start| {
                    let mut node = start;
                    step.clone().enumerate().find_map(|(i, step)| {
                        node = self.map.get(node).expect("missing node")[step as usize];
                        node.ends_with('Z').then_some(i + 1)
                    }).expect("no end")
                })
                .reduce(lcm).unwrap()
        )
    }

    INPUT_EX { 1 part1 = 2 }
    INPUT_EX2 { 1 part1 = 6 }
    INPUT_EX3 { 2 part2 = 6 }
    INPUT { 1 part1 = 18113, 2 part2 = 12315788159977 }
}

#[derive(Clone, Copy)]
enum Dir {
    Left = 0,
    Right = 1,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}
