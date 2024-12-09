use crate::Input;
use core::iter;
use str_block::str_block;

pub const INPUTS: &[Input] = &[
    Input::Hashed("4019b513957b1f8761dc6de01ae1e0be1c6159c6d3b7cbc1b5cb22ddc9b70be8"),
    Input::Inline(
        "example",
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
        Some(18),
        Some(9),
    ),
];

pub const LX: u32 = 1 << 0;
pub const UX: u32 = 1 << 1;
pub const RX: u32 = 1 << 2;
pub const LS: u32 = 1 << 3;
pub const US: u32 = 1 << 4;
pub const RS: u32 = 1 << 5;
pub const LXM: u32 = 1 << 6;
pub const UXM: u32 = 1 << 7;
pub const RXM: u32 = 1 << 8;
pub const LSA: u32 = 1 << 9;
pub const USA: u32 = 1 << 10;
pub const RSA: u32 = 1 << 11;
pub const LXMA: u32 = 1 << 12;
pub const UXMA: u32 = 1 << 13;
pub const RXMA: u32 = 1 << 14;
pub const LSAM: u32 = 1 << 15;
pub const USAM: u32 = 1 << 16;
pub const RSAM: u32 = 1 << 17;

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
    let mut buf = [0; 256];
    let mut lines = input.lines().map(|line| line.as_bytes());
    let line = lines.next().unwrap();
    let len = line.len();
    let mut count = 0;
    let mut n = 0;
    for i in 0..len {
        match line[i] {
            b'X' => {
                count += line[i + 1..].starts_with(b"MAS") as u32;
                if i > 0 {
                    buf[i - 1] |= RX;
                }
                buf[i] = n | UX;
                n = LX
            }
            b'S' => {
                count += line[i + 1..].starts_with(b"AMX") as u32;
                if i > 0 {
                    buf[i - 1] |= RS;
                }
                buf[i] = n | US;
                n = LS
            }
            _ => {
                buf[i] = n;
                n = 0
            }
        }
    }
    for line in lines {
        let mut n = 0;
        for i in 0..len {
            match line[i] {
                b'X' => {
                    count += (buf[i] & (LSAM | USAM | RSAM)).count_ones()
                        + line[i + 1..].starts_with(b"MAS") as u32;
                    if i > 0 {
                        buf[i - 1] |= RX;
                    }
                    buf[i] = n | UX;
                    n = LX;
                }
                b'S' => {
                    count += (buf[i] & (LXMA | UXMA | RXMA)).count_ones()
                        + line[i + 1..].starts_with(b"AMX") as u32;
                    if i > 0 {
                        buf[i - 1] |= RS;
                    }
                    buf[i] = n | US;
                    n = LS;
                }
                b'M' => {
                    let pn = n;
                    if i > 0 {
                        buf[i - 1] |= (buf[i] & (RX | RSA)) << 6;
                    }
                    n = (buf[i] & (LX | LSA)) << 6;
                    buf[i] = pn | ((buf[i] & (UX | USA)) << 6);
                }
                b'A' => {
                    let pn = n;
                    if i > 0 {
                        buf[i - 1] |= (buf[i] & (RXM | RS)) << 6;
                    }
                    n = (buf[i] & (LXM | LS)) << 6;
                    buf[i] = pn | ((buf[i] & (UXM | US)) << 6);
                }
                _ => {
                    buf[i] = n;
                    n = 0
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let mut lines = input.lines().map(|line| line.as_bytes());
    let mut buf = [lines.next().unwrap(), lines.next().unwrap()];
    let len = buf[0].len();
    let mut count = 0;
    let mut i = 0;
    for line in lines {
        for mi in match_indices(&buf[1 - i][1..len - 1], |b| b == b'A') {
            let prev = &buf[i];
            count += (((prev[mi] == b'M' && line[mi + 2] == b'S')
                || (prev[mi] == b'S' && line[mi + 2] == b'M'))
                && ((prev[mi + 2] == b'M' && line[mi] == b'S')
                    || (prev[mi + 2] == b'S' && line[mi] == b'M'))) as u32;
        }
        buf[i] = line;
        i = 1 - i;
    }
    count
}
