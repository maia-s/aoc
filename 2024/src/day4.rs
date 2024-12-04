use crate::Conf;
use core::iter;
use str_block::str_block;

pub const INPUT: Conf<u32> = Conf::new(include_str!("day4.txt"), 2633, 0);

pub const EX: Conf<u32> = Conf::new(
    str_block! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "},
    18,
    0,
);

fn match_indices<'a>(
    bytes: &'a [u8],
    f: impl Fn(u8) -> bool + 'a,
) -> impl Iterator<Item = usize> + 'a {
    let mut i = 0;
    iter::from_fn(move || {
        bytes[i..].iter().position(|&b| f(b)).map(|p| {
            let p = p + i;
            i = p + 1;
            p
        })
    })
}

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines().map(|line| line.as_bytes()).enumerate();
    let mut buf = [b"" as &[u8]; 4];
    let mut count = 0;
    for (i, line) in lines.by_ref().take(3) {
        buf[i] = line;
        for mi in match_indices(line, |b| matches!(b, b'X' | b'S')) {
            if line[mi] == b'X' {
                count += line[mi + 1..].starts_with(b"MAS") as u32;
            } else {
                count += line[mi + 1..].starts_with(b"AMX") as u32;
            }
        }
    }
    let len = buf[0].len();
    for (i, line) in lines {
        buf[i & 3] = line;
        for mi in match_indices(line, |b| matches!(b, b'X' | b'S')) {
            if line[mi] == b'X' {
                count += line[mi + 1..].starts_with(b"MAS") as u32;
                if mi > 2 {
                    count += (buf[(i - 1) & 3][mi - 1] == b'M'
                        && buf[(i - 2) & 3][mi - 2] == b'A'
                        && buf[(i - 3) & 3][mi - 3] == b'S') as u32;
                }
                count += (buf[(i - 1) & 3][mi] == b'M'
                    && buf[(i - 2) & 3][mi] == b'A'
                    && buf[(i - 3) & 3][mi] == b'S') as u32;
                if mi < len - 3 {
                    count += (buf[(i - 1) & 3][mi + 1] == b'M'
                        && buf[(i - 2) & 3][mi + 2] == b'A'
                        && buf[(i - 3) & 3][mi + 3] == b'S') as u32;
                }
            } else {
                count += line[mi + 1..].starts_with(b"AMX") as u32;
                if mi > 2 {
                    count += (buf[(i - 1) & 3][mi - 1] == b'A'
                        && buf[(i - 2) & 3][mi - 2] == b'M'
                        && buf[(i - 3) & 3][mi - 3] == b'X') as u32;
                }
                count += (buf[(i - 1) & 3][mi] == b'A'
                    && buf[(i - 2) & 3][mi] == b'M'
                    && buf[(i - 3) & 3][mi] == b'X') as u32;
                if mi < len - 3 {
                    count += (buf[(i - 1) & 3][mi + 1] == b'A'
                        && buf[(i - 2) & 3][mi + 2] == b'M'
                        && buf[(i - 3) & 3][mi + 3] == b'X') as u32;
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    0
}
