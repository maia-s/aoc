use aoc_2023::{aoc, str_block};

const INPUT: &str = include_str!("day-9.txt");

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

    test day9_example(INPUT_EX, 114);
}

#[allow(clippy::ptr_arg)]
fn find_sequence(seq: &Vec<isize>) -> isize {
    fn find_sequence_r(seq: &mut [isize]) -> isize {
        let mut all_zeroes = seq[0] == 0;
        for i in 0..seq.len() - 1 {
            if seq[i + 1] != 0 {
                all_zeroes = false;
            }
            seq[i] = seq[i + 1] - seq[i];
        }
        if all_zeroes {
            0
        } else {
            let last = seq.len() - 1;
            seq[last] + find_sequence_r(&mut seq[..last])
        }
    }
    find_sequence_r(&mut seq.clone())
}
