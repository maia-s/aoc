use crate::Conf;
use core::iter;
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
            if b == b')' {
                return Some(l * r);
            }
            break;
        }
        r = r * 10 + v as u32
    }
    None
}

fn matches(bytes: &[u8]) -> impl Iterator<Item = usize> + '_ {
    let mut i = 0;
    iter::from_fn(move || {
        'find: while let Some(j) = bytes[i..].iter().position(|b| matches!(b, b'm' | b'd')) {
            let j = i + j;
            i = j + 1;
            if bytes[j..].starts_with(b"mul(") {
                return Some(j);
            } else if bytes[j..].starts_with(b"don't()") {
                let mut k = j + 7;
                while let Some(m) = bytes[k..].iter().position(|&b| b == b'd') {
                    let m = m + k;
                    if bytes[m..].starts_with(b"do()") {
                        i = m + 4;
                        continue 'find;
                    }
                    k = m + 1;
                }
                break;
            }
        }
        None
    })
}

pub fn part1(input: &str) -> u32 {
    input
        .match_indices("mul(")
        .filter_map(|(i, _)| mul(&input.as_bytes()[i + 4..]))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    matches(input).filter_map(|i| mul(&input[i + 4..])).sum()
}
