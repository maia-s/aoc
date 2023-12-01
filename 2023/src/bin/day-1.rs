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
    struct Day1 {
        sum: usize,
        sum2: usize,
    }

    self(input) {
        const WORDS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        let mut sum = 0;
        let mut sum2 = 0;

        for line in input.lines() {
            let line = line.trim();

            let nums: Vec<usize> = line.chars().filter_map(|c| match c {
                '0'..='9' => Some(c as usize - b'0' as usize),
                _ => None
            }).collect();

            let mut nums2 = Vec::new();
            'lines: for i in 0..line.len() {
                let rest = &line[i..];
                nums2.push('num: {
                    match rest.chars().next().unwrap() {
                        c @ '0'..='9' => break 'num c as usize - b'0' as usize,
                        _ => {
                            for (wi, word) in WORDS.iter().enumerate() {
                                if rest.starts_with(word) {
                                    break 'num wi + 1;
                                }
                            }
                            continue 'lines;
                        }
                    }
                });
            }

            sum += nums.first().unwrap_or(&0) * 10 + nums.last().unwrap_or(&0);
            sum2 += nums2.first().unwrap_or(&0) * 10 + nums2.last().unwrap_or(&0);
        }
        Ok(Day1 { sum, sum2 })
    }

    part1 usize {
        Ok(self.sum)
    }

    part2 usize {
        Ok(self.sum2)
    }

    input = INPUT;

    test day1_example(INPUT_EX, 142, 142);
    test day1_example2(INPUT_EX2, 209, 281);
    test day1(INPUT, 55712, 55413);
}
