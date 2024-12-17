use crate::Input;
use core::simd::{
    cmp::{SimdOrd, SimdPartialOrd},
    mask8x32, mask8x64, u8x32, u8x64,
};
use str_block::str_block;

pub fn inputs() -> Vec<Input> {
    vec![
        Input::Hashed("a494403f567adfd2dc6524b2ffe0a1d2e8d3153b8352ca6ca80685e9d39af088"),
        Input::Inline(
            "example",
            str_block! {"
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
            "},
            Some(143),
            Some(123),
        ),
    ]
}

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
    let mut map = [[0_u64; 2]; 100];
    for line in sec1.split(|&b| b == b'\n') {
        let (a, b) = parse_a(line);
        let m = &mut map[b as usize];
        m[(a >= 64) as usize] |= 1_u64 << (a & 63);
    }
    let mut sum = 0;
    'sec2: for mut line in sec2.split(|&b| b == b'\n') {
        let mut history = [0; 24];
        let num = parse_b0(&mut line);
        history[0] = num;
        let mut disallow = map[num as usize];
        let mut i = 0;
        while let Some(num) = parse_b(&mut line) {
            i += 1;
            history[i] = num;
            if (disallow[(num >= 64) as usize] & (1_u64 << num)) != 0 {
                continue 'sec2;
            }
            let m = map[num as usize];
            disallow[0] |= m[0];
            disallow[1] |= m[1];
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
        let mut order = [u8x64::splat(32); 2];
        let mut history = u8x32::default();
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
            let ii = if before_i < 32 {
                reordered = true;
                history = mask8x32::from_bitmask(!((1 << before_i) - 1))
                    .select(history.rotate_elements_right::<1>(), history);
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
