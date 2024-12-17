#![allow(clippy::enum_variant_names)]

use crate::Input;
use str_block::str_block;

pub fn inputs() -> Vec<Input<u64>> {
    vec![
        Input::Hashed("30e1c6a2e8425ba49f7194d019dde4f9179e6994c68681ae0c48af82c26c29b4"),
        Input::Inline(
            "example",
            str_block! {"
                190: 10 19
                3267: 81 40 27
                83: 17 5
                156: 15 6
                7290: 6 8 6 15
                161011: 16 10 13
                192: 17 8 14
                21037: 9 7 18 13
                292: 11 6 16 20
            "},
            Some(3749),
            Some(11387),
        ),
    ]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    MulTo(u64),
    AddTo(u64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op2 {
    MulTo(u64),
    AddTo(u64),
    CatTo(u64),
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
    let mut ops = Vec::with_capacity(12);
    let mut total = 0;
    'lines: for line in input.as_bytes().trim_ascii_end().split(|&b| b == b'\n') {
        if let Some(target) = parse_line(line, &mut nums) {
            let mut subtarget = target;
            ops.clear();
            'sums: while ops.len() + 1 < nums.len() {
                let mut full = ops.len() + 2 == nums.len();
                let num = nums[nums.len() - ops.len() - 1] as u64;
                let pop = if subtarget >= num {
                    if subtarget % num == 0 {
                        ops.push(Op::MulTo(subtarget));
                        subtarget /= num;
                    } else {
                        ops.push(Op::AddTo(subtarget));
                        subtarget -= num;
                    }
                    full && subtarget != nums[0] as u64
                } else {
                    full = false;
                    true
                };
                if pop {
                    while let Some(pop) = ops.pop() {
                        match pop {
                            Op::MulTo(t) => {
                                subtarget = t;
                                let num = nums[nums.len() - ops.len() - 1] as u64;
                                if subtarget >= num {
                                    ops.push(Op::AddTo(subtarget));
                                    subtarget -= num;
                                    if !(full && subtarget != nums[0] as u64) {
                                        continue 'sums;
                                    }
                                } else {
                                    full = false;
                                }
                            }
                            Op::AddTo(_) => full = false,
                        }
                    }
                    continue 'lines;
                }
            }
            total += target;
        }
    }
    total
}

pub fn part2(input: &str) -> u64 {
    let mut nums = Vec::with_capacity(12);
    let mut ops = Vec::with_capacity(12);
    let mut total = 0;
    'lines: for line in input.as_bytes().trim_ascii_end().split(|&b| b == b'\n') {
        if let Some(target) = parse_line(line, &mut nums) {
            let mut subtarget = target;
            ops.clear();
            'sums: while ops.len() + 1 < nums.len() {
                let mut full = ops.len() + 2 == nums.len();
                let num = nums[nums.len() - ops.len() - 1] as u64;
                let ndig = num.ilog10() + 1;
                let pow = 10_u64.pow(ndig);
                let pop = if subtarget % pow == num {
                    ops.push(Op2::CatTo(subtarget));
                    subtarget /= pow;
                    full && subtarget != nums[0] as u64
                } else if subtarget >= num {
                    if subtarget % num == 0 {
                        ops.push(Op2::MulTo(subtarget));
                        subtarget /= num;
                    } else {
                        ops.push(Op2::AddTo(subtarget));
                        subtarget -= num;
                    }
                    full && subtarget != nums[0] as u64
                } else {
                    full = false;
                    true
                };
                if pop {
                    while let Some(pop) = ops.pop() {
                        match pop {
                            Op2::CatTo(t) => {
                                subtarget = t;
                                let num = nums[nums.len() - ops.len() - 1] as u64;
                                if subtarget >= num {
                                    if subtarget % num == 0 {
                                        ops.push(Op2::MulTo(subtarget));
                                        subtarget /= num;
                                    } else {
                                        ops.push(Op2::AddTo(subtarget));
                                        subtarget -= num;
                                    }
                                    if !(full && subtarget != nums[0] as u64) {
                                        continue 'sums;
                                    }
                                } else {
                                    full = false;
                                }
                            }
                            Op2::MulTo(t) => {
                                subtarget = t;
                                let num = nums[nums.len() - ops.len() - 1] as u64;
                                if subtarget >= num {
                                    ops.push(Op2::AddTo(subtarget));
                                    subtarget -= num;
                                    if !(full && subtarget != nums[0] as u64) {
                                        continue 'sums;
                                    }
                                } else {
                                    full = false;
                                }
                            }
                            Op2::AddTo(_) => full = false,
                        }
                    }
                    continue 'lines;
                }
            }
            total += target;
        }
    }
    total
}
