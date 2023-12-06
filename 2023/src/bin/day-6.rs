use aoc_2023::{aoc, str_block};

const INPUT: &str = include_str!("day-6.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
Time:      7  15   30
Distance:  9  40  200
"};

aoc! {
    struct Day6 {
        races: Vec<(usize, usize)>,
    }

    self(input = INPUT) {
        let mut line = input.lines().map(|line| line.split_ascii_whitespace().skip(1).map(str::parse));
        let time = line.next().ok_or("missing times")?;
        let distance = line.next().ok_or("missing distances")?;
        Ok(Self { races: time.zip(distance).map(|(t, d)| match (t, d) {
            (Ok(t), Ok(d)) => Ok((t, d)),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }).collect::<Result<_, _>>()? })
    }

    part1 usize {
        Ok(self.races.iter().map(|race|
            (1..race.0).map(|t| t * (race.0 - t)).filter(|&d| d > race.1).count()
        ).product())
    }

    test day6_example(INPUT_EX, 288);
}
