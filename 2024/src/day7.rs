use crate::{Conf, Input};
use str_block::str_block;

pub const INPUT: Conf<u64> = Conf::new(
    Input::FileHash("30e1c6a2e8425ba49f7194d019dde4f9179e6994c68681ae0c48af82c26c29b4"),
    267566105056,
    0,
);

pub const EX: Conf<u64> = Conf::new(
    Input::Str(str_block! {"
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
    "}),
    3749,
    0,
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    MulFrom(u64),
    AddFrom(u64),
}

fn int(input: &mut &[u8]) -> Option<u64> {
    if let Some(b) = input.first() {
        *input = &input[1..];
        let mut v = b.wrapping_sub(b'0') as u64;
        while let Some(b) = input.first() {
            *input = &input[1..];
            let digit = b.wrapping_sub(b'0');
            if digit > 9 {
                break;
            }
            v = v * 10 + digit as u64;
        }
        return Some(v);
    }
    None
}

fn parse_line(mut input: &[u8], out: &mut Vec<u32>) -> Option<u64> {
    out.clear();
    int(&mut input).inspect(|_| {
        input = &input[1..];
        while let Some(int) = int(&mut input) {
            out.push(int as u32);
        }
    })
}

pub fn part1(input: &str) -> u64 {
    let mut nums = Vec::with_capacity(12);
    let mut sums = Vec::with_capacity(12);
    let mut total = 0;
    'lines: for line in input.as_bytes().trim_ascii_end().split(|&b| b == b'\n') {
        if let Some(target) = parse_line(line, &mut nums) {
            let mut sum = nums[0] as u64;
            sums.clear();
            'sums: while sums.len() + 1 < nums.len() {
                let last = sums.len() + 2 == nums.len();
                let arg = nums[sums.len() + 1] as u64;
                let s = sum * arg;
                if if last { s == target } else { s <= target } {
                    sums.push(Op::MulFrom(sum));
                    sum = s;
                } else {
                    let s = sum + arg;
                    if if last { s == target } else { s <= target } {
                        sums.push(Op::AddFrom(sum));
                        sum = s;
                    } else {
                        while let Some(op) = sums.pop() {
                            if let Op::MulFrom(psum) = op {
                                sum = psum;
                                let s = sum + nums[sums.len() + 1] as u64;
                                if s <= target {
                                    sums.push(Op::AddFrom(sum));
                                    sum = s;
                                } else {
                                    continue;
                                }
                                continue 'sums;
                            }
                        }
                        continue 'lines;
                    }
                }
            }
            total += target;
        }
    }
    total
}

pub fn part2(input: &str) -> u64 {
    0
}
