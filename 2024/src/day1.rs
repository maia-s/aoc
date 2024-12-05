use crate::{Conf, Input};
use str_block::str_block;

pub const INPUT: Conf<u32> = Conf::new(
    Input::FileHash("bd3e2df596a877265fe4a28b626ac1ed30239c051e6623b4c852be317288fe1a"),
    2196996,
    23655822,
);

pub const EX: Conf<u32> = Conf::new(
    Input::Str(str_block! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "}),
    11,
    31,
);

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

pub fn part1(input: &str) -> u32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = Vec::with_capacity(1000);
    for line in input.lines() {
        let (l, r) = parse_line(line);
        left.push(l);
        right.push(r);
    }
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut left = Vec::with_capacity(1000);
    let mut right = vec![0; 100_000];
    for line in input.lines() {
        let (l, r) = parse_line(line);
        left.push(l);
        right[r as usize] += 1;
    }
    left.into_iter().map(|i| i * right[i as usize]).sum()
}
