use crate::Conf;
use core::{hint::unreachable_unchecked, iter};
use str_block::str_block;

pub const INPUT: Conf<u32> = Conf::new(include_str!("day3.txt"), 181345830, 98729041);

pub const EX: Conf<u32> = Conf::new(
    str_block! {"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "},
    161,
    161,
);

pub const EX2: Conf<u32> = Conf::new(
    str_block! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "},
    161,
    48,
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

fn matches(bytes: &[u8]) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut i = 0;
    iter::from_fn(move || {
        while let Some(j) = bytes[i..].iter().position(|b| matches!(b, b'm' | b'd')) {
            let j = i + j;
            i = j + 1;
            for (alti, alt) in [b"mul(" as &[u8], b"do()", b"don't()"]
                .into_iter()
                .enumerate()
            {
                if bytes[j..].starts_with(alt) {
                    return Some((alti, j));
                }
            }
        }
        None
    })
}

pub fn part1(input: &str) -> u32 {
    input
        .match_indices("mul(")
        .filter_map(|(i, _)| {
            let i = i + 4;
            input[i..]
                .find(')')
                .and_then(|j| mul(&input.as_bytes()[i..i + j]))
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut enabled = true;
    matches(input.as_bytes())
        .filter_map(|(alti, i)| match alti {
            0 => {
                if enabled {
                    let i = i + 4;
                    input[i..]
                        .find(')')
                        .and_then(|j| mul(&input.as_bytes()[i..i + j]))
                } else {
                    None
                }
            }
            1 => {
                enabled = true;
                None
            }
            2 => {
                enabled = false;
                None
            }
            _ => unsafe { unreachable_unchecked() },
        })
        .sum()
}
