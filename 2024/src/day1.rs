use core::str::FromStr;

struct Part1 {
    left: Vec<isize>,
    right: Vec<isize>,
}

struct Part2 {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl FromStr for Part1 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in s.lines() {
            let (l, r) = line.split_once("   ").unwrap();
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
        Ok(Self { left, right })
    }
}

impl FromStr for Part2 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::new();
        let mut right = vec![0; 100_000];
        for line in s.lines() {
            let (l, r) = line.split_once("   ").unwrap();
            left.push(l.parse().unwrap());
            right[r.parse::<usize>().unwrap()] += 1;
        }
        Ok(Self { left, right })
    }
}

pub fn part1(input: &str) -> usize {
    let mut input: Part1 = input.parse().unwrap();
    input.left.sort_unstable();
    input.right.sort_unstable();
    input
        .left
        .into_iter()
        .zip(input.right)
        .map(|(l, r)| (l - r).unsigned_abs())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let input: Part2 = input.parse().unwrap();
    input.left.into_iter().map(|i| i * input.right[i]).sum()
}
