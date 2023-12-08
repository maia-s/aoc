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

    part2 usize {
        // so not stated in the puzzle description but apparently something we can rely on
        // is that each start only has one possible end and repeats after that with the same
        // cycle length each time
        let step = self.path.iter().copied().cycle();
        let cycles = self.map.keys().copied().filter(|key| key.ends_with('A'))
            .map(|start| {
                let mut node = start;
                step.clone().enumerate().find_map(|(i, step)| {
                    node = self.map.get(node).expect("missing node")[step as usize];
                    node.ends_with('Z').then_some(i + 1)
                }).expect("no end")
            }).collect::<Vec<_>>();
        let mut lcm = cycles.clone();
        let lcm = loop {
            let mut min = usize::MAX;
            let mut min_i = 0;
            let mut max = 0;
            for (i, &n) in lcm.iter().enumerate() {
                if min > n {
                    min = n;
                    min_i = i;
                }
                max = max.max(n);
            }
            if min == max {
                break min;
            }
            lcm[min_i] += cycles[min_i];
        };
        Ok(lcm)
    }

    test day8_example(INPUT_EX, 2);
    test day8_example2(INPUT_EX2, 6);
    test day8_example3(INPUT_EX3,, 6);
    test day8(INPUT, 18113, 12315788159977);
}

#[derive(Clone, Copy)]
enum Dir {
    Left = 0,
    Right = 1,
}
