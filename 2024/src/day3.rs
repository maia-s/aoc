use core::str;

use crate::Conf;
use str_block::str_block;

pub const INPUT: Conf<u32> = Conf::new(include_str!("day3.txt"), 181345830, 0);

pub const EX: Conf<u32> = Conf::new(
    str_block! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "},
    161,
    0,
);

fn mul(bytes: &[u8]) -> Option<u32> {
    let mut l = 0;
    let mut r = 0;
    let mut bytes = bytes.iter().copied();
    for b in bytes.by_ref() {
        let v = b.wrapping_sub(b'0');
        if v > 9 {
            break;
        }
        l = l * 10 + v as u32
    }
    for b in bytes {
        let v = b.wrapping_sub(b'0');
        if v > 9 {
            return None;
        }
        r = r * 10 + v as u32
    }
    Some(l * r)
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.match_indices("mul(")
                .filter_map(|(i, _)| {
                    let i = i + 4;
                    line[i..]
                        .find(')')
                        .and_then(|j| mul(&line.as_bytes()[i..i + j]))
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    0
}
