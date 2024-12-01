use crate::Conf;
use core::{convert::Infallible, str::FromStr};
use str_block::str_block;

pub const INPUT: Conf<u32> = Conf::new(include_str!("day1.txt"), 2196996, 23655822);

pub const EX: Conf<u32> = Conf::new(
    str_block! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "},
    11,
    31,
);

struct Part1 {
    left: Vec<u32>,
    right: Vec<u32>,
}

struct Part2 {
    left: Vec<u32>,
    right: Vec<u32>,
}

fn parse_line(s: &str) -> (u32, u32) {
    let mut l = 0;
    let mut r = 0;
    let mut bytes = s.as_bytes().iter().copied();
    for b in bytes.by_ref() {
        let v = b.wrapping_sub(b'0');
        if v > 9 {
            break;
        }
        l = l * 10 + v as u32;
    }
    bytes.next();
    bytes.next();
    for b in bytes {
        let v = b.wrapping_sub(b'0');
        r = r * 10 + v as u32;
    }
    (l, r)
}

impl FromStr for Part1 {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::with_capacity(1000);
        let mut right = Vec::with_capacity(1000);
        for line in s.lines() {
            let (l, r) = parse_line(line);
            left.push(l);
            right.push(r);
        }
        Ok(Self { left, right })
    }
}

impl FromStr for Part2 {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::with_capacity(1000);
        let mut right = vec![0; 100_000];
        for line in s.lines() {
            let (l, r) = parse_line(line);
            left.push(l);
            right[r as usize] += 1;
        }
        Ok(Self { left, right })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut input: Part1 = input.parse().unwrap();
    input.left.sort_unstable();
    input.right.sort_unstable();
    input
        .left
        .into_iter()
        .zip(input.right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let input: Part2 = input.parse().unwrap();
    input
        .left
        .into_iter()
        .map(|i| i * input.right[i as usize])
        .sum()
}
