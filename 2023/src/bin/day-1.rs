const INPUT: &str = include_str!("day-1.txt");
const INPUT_EX: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

aoc_2023::aoc! {
    struct Day1 {
        sum: usize,
    }

    self(input) {
        let mut sum = 0;
        for line in input.lines() {
            let nums: Vec<usize> = line.trim().chars().filter_map(|c| match c {
                '0'..='9' => Some(c as usize - b'0' as usize),
                _ => None
            }).collect();
            sum += nums.first().unwrap() * 10 + nums.last().unwrap();
        }
        Ok(Day1 { sum })
    }

    part1 usize {
        Ok(self.sum)
    }

    part2 usize {
        Ok(0)
    }

    input = INPUT;

    test day1_example(INPUT_EX, 142, 0);
}
