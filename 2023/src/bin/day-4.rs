use std::collections::HashSet;

const INPUT: &str = include_str!("day-4.txt");

#[allow(dead_code)]
const INPUT_EX: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

aoc_2023::aoc! {
    struct Day4 {
        points: usize,
    }

    self(input = INPUT) {
        let mut points = 0;
        for card in input.lines() {
            let (_, card) = card.split_once(": ").ok_or("malformed line")?;
            let (win, have) = card.split_once(" | ").ok_or("missing `|`")?;
            let win = win.split_ascii_whitespace().map(|x| x.parse()).collect::<Result<HashSet<usize>, _>>()?;
            let matches = have.split_ascii_whitespace().filter_map(
                |x| win.contains(&x.parse().ok()?).then_some(())
            ).count();
            if matches > 0 {
                points += 1 << (matches - 1);
            }
        }
        Ok(Self { points })
    }

    part1 usize {
        Ok(self.points)
    }

    test day4_example(INPUT_EX, 13);
    test day4(INPUT, 26346);
}
