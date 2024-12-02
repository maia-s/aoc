use crate::Conf;
use str_block::str_block;

pub const INPUT: Conf<u32> = Conf::new(include_str!("day2.txt"), 383, 436);
pub const EX: Conf<u32> = Conf::new(
    str_block! {"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "},
    2,
    4,
);
pub const EDGE_CASE: Conf<u32> = Conf::new(
    str_block! {"
        25 22 19 21 20 17 14 13
    "},
    0,
    1,
);

fn parse(s: &str) -> i32 {
    s.as_bytes()
        .iter()
        .fold(0, |acc, i| acc * 10 + (i - b'0') as i32)
}

pub fn part1(input: &str) -> u32 {
    let mut nsafe = 0;
    'lines: for line in input.lines() {
        let mut nums = line.split_ascii_whitespace().map(parse);
        let first = nums.next().unwrap();
        let mut prev = nums.next().unwrap();
        let diff = prev - first;
        if diff == 0 || diff.abs() > 3 {
            continue;
        }
        let ascending = diff > 0;
        for num in nums {
            let diff = num - prev;
            prev = num;
            if if ascending { diff <= 0 } else { diff >= 0 } || diff.abs() > 3 {
                continue 'lines;
            }
        }
        nsafe += 1;
    }
    nsafe
}

fn check(nums: &[i32]) -> bool {
    let first = nums[0];
    let mut prev = nums[1];
    let diff = prev - first;
    if diff == 0 || diff.abs() > 3 {
        return check_with_dampen(nums, 0) || check_with_dampen(nums, 1);
    }
    let ascending = diff > 0;
    let mut try_dampen = 0;
    for (i, &num) in nums[2..].iter().enumerate() {
        let diff = num - prev;
        if if ascending { diff <= 0 } else { diff >= 0 } || diff.abs() > 3 {
            if try_dampen == 0 {
                try_dampen = i + 1;
                continue;
            } else {
                return check_with_dampen(nums, try_dampen)
                    || (try_dampen == 1 && check_with_dampen(nums, 0));
            }
        }
        prev = num;
    }
    true
}

fn check_with_dampen(nums: &[i32], dampen: usize) -> bool {
    if dampen < 2 {
        let first = nums[(dampen == 0) as usize];
        let mut prev = nums[2];
        let diff = prev - first;
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        let ascending = diff > 0;
        for &num in nums[3..].iter() {
            let diff = num - prev;
            if if ascending { diff <= 0 } else { diff >= 0 } || diff.abs() > 3 {
                return false;
            }
            prev = num;
        }
    } else {
        let first = nums[0];
        let mut prev = nums[1];
        let diff = prev - first;
        let ascending = diff > 0;
        for (i, &num) in nums[2..].iter().enumerate() {
            if i + 2 == dampen {
                continue;
            }
            let diff = num - prev;
            if if ascending { diff <= 0 } else { diff >= 0 } || diff.abs() > 3 {
                return false;
            }
            prev = num;
        }
    }
    true
}

pub fn part2(input: &str) -> u32 {
    let mut nsafe = 0;
    for line in input.lines() {
        let mut nums = [0; 10];
        let mut n = 0;
        for num in line.split_ascii_whitespace().map(parse) {
            nums[n] = num;
            n += 1;
        }
        nsafe += check(&nums[..n]) as u32;
    }
    nsafe
}
