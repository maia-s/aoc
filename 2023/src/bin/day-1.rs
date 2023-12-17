use aoc_2023::str_block;

const INPUT: &str = include_str!("day-1.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"};

#[allow(dead_code)]
const INPUT_EX2: &str = str_block! {"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"};

fn get_number(mut it: impl Iterator<Item = usize>) -> usize {
    let first = it.next().unwrap_or(0);
    let last = it.last().unwrap_or(first);
    first * 10 + last
}

aoc_2023::aoc! {
    struct Day1<'a> {
        input: &'a str,
    }

    self(input = INPUT) {
        Ok(Day1 { input })
    }

    1 part1 usize {
        Ok(self.input.lines().map(|line| {
            get_number(line.trim().chars().filter_map(|c| match c {
                '0'..='9' => Some(c as usize - '0' as usize),
                _ => None
            }))
        }).sum())
    }

    2 part2 usize {
        const WORDS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        Ok(self.input.lines().map(|line| {
            let line = line.trim();
            get_number((0..line.len()).filter_map(|i| {
                let rest = &line[i..];
                match rest.chars().next().unwrap() {
                    c @ '0'..='9' => Some(c as usize - '0' as usize),
                    _ => {
                        for (wi, word) in WORDS.iter().enumerate() {
                            if rest.starts_with(word) {
                                return Some(wi + 1);
                            }
                        }
                        None
                    }
                }
            }))
        }).sum())
    }

    INPUT_EX { 1 part1 = 142, 2 part2 = 142 }
    INPUT_EX2 { 1 part1 = 209, 2 part2 = 281 }
    INPUT { 1 part1 = 55712, 2 part2 = 55413 }
}
