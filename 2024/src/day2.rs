use crate::Conf;
use str_block::str_block;

pub const INPUT: Conf<u32> = Conf::new(include_str!("day2.txt"), 383, 0);
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
    0,
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

pub fn part2(input: &str) -> u32 {
    0
}
