use aoc_2023::aoc;

const INPUT: &str = include_str!("day-15.txt");

#[allow(dead_code)]
const INPUT_EX: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

aoc! {
    struct Day15<'a> {
        words: Vec<&'a str>
    }

    self(input = INPUT) {
        Ok(Self { words: input.trim().split(',').collect() })
    }

    part1 usize {
        Ok(self.words.iter().map(|s| hash(s.as_bytes()) as usize).sum())
    }

    test day15_example(INPUT_EX, 1320);
    test day15(INPUT, 517315);
}

fn hash(s: &[u8]) -> u8 {
    s.iter()
        .fold(0, |hash, &b| hash.wrapping_add(b).wrapping_mul(17))
}
