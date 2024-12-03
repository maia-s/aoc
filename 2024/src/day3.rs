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

fn mul_with_len(bytes: &[u8]) -> Option<(usize, u32)> {
    let mut l = 0;
    let mut r = 0;
    let mut bytes = bytes.iter().copied().enumerate();
    for (_, b) in bytes.by_ref() {
        let v = b.wrapping_sub(b'0');
        if v > 9 {
            break;
        }
        l = l * 10 + v as u32
    }
    for (i, b) in bytes {
        let v = b.wrapping_sub(b'0');
        if v > 9 {
            if b == b')' {
                return Some((i + 1, l * r));
            }
            break;
        }
        r = r * 10 + v as u32
    }
    None
}

fn matches(bytes: &[u8]) -> impl Iterator<Item = u32> + '_ {
    let mut i = 0;
    iter::from_fn(move || {
        'find: while let Some(j) = bytes[i..].iter().position(|b| matches!(b, b'm' | b'd')) {
            i += j + 1;
            if bytes[i..].starts_with(b"ul(") {
                i += 3;
                if let Some((n, val)) = mul_with_len(&bytes[i..]) {
                    i += n;
                    return Some(val);
                }
                continue;
            } else if bytes[i..].starts_with(b"on't()") {
                let mut k = i + 6;
                while let Some(m) = bytes[k..].iter().position(|&b| b == b'd') {
                    let m = m + k + 1;
                    if bytes[m..].starts_with(b"o()") {
                        i = m + 3;
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
    matches(input.as_bytes()).sum()
}
