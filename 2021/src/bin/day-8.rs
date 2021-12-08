use std::str::FromStr;

const INPUT: &str = include_str!("day-8.input");

#[derive(Clone, Copy)]
struct Digit(u8);

impl Digit {
    fn number_of_segments(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = 0;
        for c in s.chars() {
            let bit = (c as usize - 'a' as usize) as usize;
            assert!(bit < 7);
            segments |= 1 << bit;
        }
        Ok(Self(segments))
    }
}

struct Digits(Vec<Digit>);

impl FromStr for Digits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim().split(' ').map(|s| s.parse().unwrap()).collect(),
        ))
    }
}

fn main() {
    let input: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let (signals, segments) = line.split_once('|').unwrap();
            (
                signals.parse::<Digits>().unwrap().0,
                segments.parse::<Digits>().unwrap().0,
            )
        })
        .collect();

    // part 1
    println!(
        "part 1: {}",
        input
            .iter()
            .map(|(_, s)| s
                .iter()
                .map(Digit::number_of_segments)
                .filter(|n| matches!(n, 2 | 3 | 4 | 7))
                .count())
            .sum::<usize>()
    );
}
