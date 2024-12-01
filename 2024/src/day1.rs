use core::str::FromStr;

struct Day1 {
    left: Vec<isize>,
    right: Vec<isize>,
}

impl FromStr for Day1 {
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

pub fn part1(input: &str) -> usize {
    let mut input: Day1 = input.parse().unwrap();
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
    todo!()
}
