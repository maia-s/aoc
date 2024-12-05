use crate::{Conf, Input};
use core::simd::{
    cmp::{SimdOrd, SimdPartialOrd},
    mask8x64, u8x64,
};
use str_block::str_block;

pub const INPUT: Conf = Conf::new(
    Input::FileHash("a494403f567adfd2dc6524b2ffe0a1d2e8d3153b8352ca6ca80685e9d39af088"),
    5747,
    5502,
);

pub const EX: Conf = Conf::new(
    Input::Str(str_block! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "}),
    143,
    123,
);

fn parse_a(input: &[u8]) -> (u8, u8) {
    (
        (input[0] - b'0') * 10 + (input[1] - b'0'),
        (input[3] - b'0') * 10 + (input[4] - b'0'),
    )
}

fn parse_b0(input: &mut &[u8]) -> u8 {
    let v = (input[0] - b'0') * 10 + (input[1] - b'0');
    *input = &input[2..];
    v
}

fn parse_b(input: &mut &[u8]) -> Option<u8> {
    if input.len() > 2 {
        let v = (input[1] - b'0') * 10 + (input[2] - b'0');
        *input = &input[3..];
        Some(v)
    } else {
        None
    }
}

pub fn part1(input: &str) -> u32 {
    let (sec1, sec2) = input.split_once("\n\n").unwrap();
    let (sec1, sec2) = (sec1.as_bytes(), sec2.as_bytes().trim_ascii_end());
    let mut map = [[0; 25]; 100];
    for line in sec1.split(|&b| b == b'\n') {
        let (a, b) = parse_a(line);
        let m = &mut map[b as usize];
        m[0] += 1;
        m[m[0] as usize] = a;
    }
    let mut sum = 0;
    'sec2: for mut line in sec2.split(|&b| b == b'\n') {
        let mut disallow = [false; 100];
        let mut history = [0; 24];
        let num = parse_b0(&mut line);
        history[0] = num;
        let m = map[num as usize];
        for &i in &m[1..m[0] as usize + 1] {
            disallow[i as usize] = true;
        }
        let mut i = 0;
        while let Some(num) = parse_b(&mut line) {
            i += 1;
            history[i] = num;
            if disallow[num as usize] {
                continue 'sec2;
            }
            let m = map[num as usize];
            for &i in &m[1..m[0] as usize + 1] {
                disallow[i as usize] = true;
            }
        }
        sum += history[i / 2] as u32;
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let (sec1, sec2) = input.split_once("\n\n").unwrap();
    let (sec1, sec2) = (sec1.as_bytes(), sec2.as_bytes().trim_ascii_end());
    let mut map = [[mask8x64::splat(false); 2]; 100];
    for line in sec1.split(|&b| b == b'\n') {
        let (a, b) = parse_a(line);
        let m = &mut map[b as usize];
        m[(a >= 64) as usize].set(a as usize & 63, true);
    }
    let mut sum = 0;
    let vone = u8x64::splat(1);
    for mut line in sec2.split(|&b| b == b'\n') {
        let mut order = [u8x64::splat(24); 2];
        let mut history = [0; 24];
        let mut reordered = false;
        let num = parse_b0(&mut line);
        history[0] = num;
        let m = map[num as usize];
        order[0] = m[0].select(u8x64::default(), order[0]);
        order[1] = m[1].select(u8x64::default(), order[1]);
        let mut i = 0;
        while let Some(num) = parse_b(&mut line) {
            i += 1;
            let before_i = order[(num >= 64) as usize][num as usize & 63] as usize;
            let ii = if before_i < 24 {
                reordered = true;
                let before_i_to = before_i + 1;
                let hp = history.as_mut_ptr();
                unsafe { hp.add(before_i).copy_to(hp.add(before_i_to), i - before_i) };
                history[before_i] = num;
                let vcmp = u8x64::splat(before_i as u8);
                order[0] = order[0].simd_ge(vcmp).select(order[0] + vone, order[0]);
                order[1] = order[1].simd_ge(vcmp).select(order[1] + vone, order[1]);
                before_i as u8
            } else {
                history[i] = num;
                i as u8
            };
            let m = map[num as usize];
            let vii = u8x64::splat(ii);
            order[0] = m[0].select(vii, order[0]).simd_min(order[0]);
            order[1] = m[1].select(vii, order[1]).simd_min(order[1]);
        }
        if reordered {
            sum += history[i / 2] as u32;
        }
    }
    sum
}
