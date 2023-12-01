const INPUT: &str = include_str!("day-1.txt");

#[allow(dead_code)]
const INPUT_EX: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[allow(dead_code)]
const INPUT_EX2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

aoc_2023::aoc! {
    struct Day1<'a> {
        input: &'a str,
    }

    self(input) {
        Ok(Day1 { input })
    }

    part1 usize {
        Ok(self.input.lines().map(|line| {
            get_number(line.trim().chars().filter_map(|c| match c {
                '0'..='9' => Some(c as usize - b'0' as usize),
                _ => None
            }))
        }).sum())
    }

    part2 usize {
        const WORDS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        Ok(self.input.lines().map(|line| {
            let line = line.trim();
            get_number((0..line.len()).filter_map(|i| {
                let rest = &line[i..];
                match rest.chars().next().unwrap() {
                    c @ '0'..='9' => Some(c as usize - b'0' as usize),
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

    input = INPUT;

    test day1_example(INPUT_EX, 142, 142);
    test day1_example2(INPUT_EX2, 209, 281);
    test day1(INPUT, 55712, 55413);
}

fn get_number(mut it: impl Iterator<Item = usize>) -> usize {
    let first = it.next().unwrap_or(0);
    let last = it.last().unwrap_or(first);
    first * 10 + last
}
