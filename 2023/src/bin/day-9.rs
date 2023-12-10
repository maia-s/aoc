use aoc_2023::{aoc, str_block};

const INPUT: &str = include_str!("day-9.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"};

aoc! {
    struct Day9 {
        values: Vec<Vec<isize>>,
    }

    self(input = INPUT) {
        Ok(Self {
            values: input.lines().map(|line| line.split(' ')
                .map(|n| n.parse().map_err(|_| "parse error"))
                .collect()).collect::<Result<_, _>>()?,
        })
    }

    part1 isize {
        Ok(self.values.iter().map(find_sequence).sum())
    }

    part2 isize {
        Ok(self.values.iter().map(find_sequence_2).sum())
    }

    test day9_example(INPUT_EX, 114, 2);
    test day9(INPUT, 1938731307, 948);
}

#[allow(clippy::ptr_arg)]
fn find_sequence(seq: &Vec<isize>) -> isize {
    let mut seq = seq.clone();
    let mut sum = 0;
    for len in (0..=seq.len()).rev() {
        let mut all_zeroes = true;
        for i in 0..len - 1 {
            seq[i] = seq[i + 1] - seq[i];
            if seq[i] != 0 {
                all_zeroes = false;
            }
        }
        sum += seq[len - 1];
        if all_zeroes {
            return sum;
        }
    }
    unreachable!();
}

#[allow(clippy::ptr_arg)]
fn find_sequence_2(seq: &Vec<isize>) -> isize {
    let mut seq = seq.clone();
    let mut sum = 0;
    let mut n = 1;
    for start in 0.. {
        let mut all_zeroes = true;
        for i in (start..seq.len() - 1).rev() {
            seq[i + 1] -= seq[i];
            if seq[i + 1] != 0 {
                all_zeroes = false;
            }
        }
        sum += n * seq[start];
        n = -n;
        if all_zeroes {
            return sum;
        }
    }
    unreachable!();
}
