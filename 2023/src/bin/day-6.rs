use aoc_2023::{aoc, str_block};

const INPUT: &str = include_str!("day-6.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
Time:      7  15   30
Distance:  9  40  200
"};

aoc! {
    struct Day6<'a> { input: &'a str }

    self(input = INPUT) {
        Ok(Self { input })
    }

    part1 usize {
        let mut line = self.input.lines().map(|line| line.split_ascii_whitespace().skip(1).map(str::parse));
        let time = line.next().ok_or("missing times")?;
        let distance = line.next().ok_or("missing distances")?;
        Ok(time.zip(distance).map(|(t, d)| match (t, d) {
            (Ok(t), Ok(d)) => Ok((t, d)),
            (Err(e), _) | (_, Err(e)) => Err(e),
        }).map(|race| race.map(|(t, d)| ways_to_win(t, d))).product::<Result<_, _>>()?)
    }

    part2 usize {
        let mut line = self.input.lines().map(|line| {
            let (_, line) = line.split_once(':').ok_or("missing `:`")?;
            line.replace(' ', "").parse().map_err(|e| format!("parse error: {e}"))
        });
        let time = line.next().ok_or("missing time")??;
        let distance = line.next().ok_or("missing distance")??;
        Ok(ways_to_win(time, distance))
    }

    test day6_example(INPUT_EX, 288, 71503);
    test day6(INPUT, 4403592, 38017587);
}

fn ways_to_win(time: usize, distance: usize) -> usize {
    (1..time)
        .map(|t| t * (time - t))
        .filter(|&d| d > distance)
        .count()
}
