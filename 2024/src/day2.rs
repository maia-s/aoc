use crate::Input;
use core::iter;
use str_block::str_block;

pub fn inputs() -> Vec<Input> {
    vec![
        Input::Hashed("cb22dec174693f6ca8cbddc14d4457f0eced03f6d2a071bc47219f8858463926"),
        Input::Hashed("06e94f091bab7e9aa3501902610210260dda0a23b4eaaf64b5f701a3a7572fdc"),
        Input::Hashed("c1f6fa369010033322fd73bef32619bcb1b1e83ad7945d4866cbe9168e00d01d"),
        Input::Hashed("738dfa8355452e64b085fbc3e20969d2af88d9c64b4d5489bedbb73190563afc"),
        Input::Inline(
            "example",
            str_block! {"
                7 6 4 2 1
                1 2 7 8 9
                9 7 6 2 1
                1 3 2 4 5
                8 6 4 4 1
                1 3 6 7 9
            "},
            Some(2),
            Some(4),
        ),
        Input::Inline(
            "edge case",
            str_block! {"
                25 22 19 21 20 17 14 13
            "},
            Some(0),
            Some(1),
        ),
    ]
}

fn parse_line(line: &str) -> impl Iterator<Item = i32> + '_ {
    let bytes = line.as_bytes();
    let mut i = 0;
    iter::from_fn(move || {
        bytes.get(i).map(|b0| {
            let b0 = (b0 - b'0') as i32;
            if let Some(b1) = bytes.get(i + 1) {
                let b1 = (b1.wrapping_sub(b'0')) as i32;
                if b1 <= 9 {
                    i += 3;
                    b0 * 10 + b1
                } else {
                    i += 2;
                    b0
                }
            } else {
                i += 1;
                b0
            }
        })
    })
}

pub fn part1(input: &str) -> u32 {
    let mut nsafe = 0;
    'lines: for line in input.lines() {
        let mut nums = parse_line(line);
        let first = nums.next().unwrap();
        let prev = nums.next().unwrap();
        let diff = prev - first;
        if diff == 0 || diff.abs() > 3 {
            continue;
        }
        let (mina, maxa) = if diff > 0 { (1, 3) } else { (-3, -1) };
        let mut min = prev + mina;
        let mut max = prev + maxa;
        for num in nums {
            if !(min..max + 1).contains(&num) {
                continue 'lines;
            }
            min = num + mina;
            max = num + maxa;
        }
        nsafe += 1;
    }
    nsafe
}

fn check(nums: &[i32]) -> bool {
    let first = nums[0];
    let prev = nums[1];
    let diff = prev - first;
    if diff == 0 || diff.abs() > 3 {
        return check_with_dampen(nums, 0) || check_with_dampen(nums, 1);
    }
    let mut try_dampen = 0;
    let (mina, maxa) = if diff > 0 { (1, 3) } else { (-3, -1) };
    let mut min = prev + mina;
    let mut max = prev + maxa;
    for (i, num) in nums[2..].iter().enumerate() {
        if !(min..max + 1).contains(num) {
            if try_dampen == 0 {
                try_dampen = i + 1;
                continue;
            } else {
                return check_with_dampen(nums, try_dampen)
                    || (try_dampen == 1 && check_with_dampen(nums, 0));
            }
        }
        min = num + mina;
        max = num + maxa;
    }
    true
}

fn check_with_dampen(nums: &[i32], dampen: usize) -> bool {
    let (rest, prev, diff) = if dampen < 2 {
        let rest = &nums[3..];
        let first = nums[(dampen == 0) as usize];
        let prev = nums[2];
        let diff = prev - first;
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        (rest, prev, diff)
    } else {
        let rest = &nums[dampen + 1..];
        let first = nums[dampen - 2];
        let prev = nums[dampen - 1];
        let diff = prev - first;
        (rest, prev, diff)
    };
    let (mina, maxa) = if diff > 0 { (1, 3) } else { (-3, -1) };
    let mut min = prev + mina;
    let mut max = prev + maxa;
    for num in rest {
        if !(min..max + 1).contains(num) {
            return false;
        }
        min = num + mina;
        max = num + maxa;
    }
    true
}

pub fn part2(input: &str) -> u32 {
    let mut nsafe = 0;
    for line in input.lines() {
        let mut nums = [0; 10];
        let mut n = 0;
        for num in parse_line(line) {
            nums[n] = num;
            n += 1;
        }
        nsafe += check(&nums[..n]) as u32;
    }
    nsafe
}
