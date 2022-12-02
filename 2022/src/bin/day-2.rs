use std::str::FromStr;

const INPUT: &str = include_str!("day-2.txt");

aoc_2022::aoc! {
    struct Day2 {
        rounds: Vec<Round>,
    }

    self(input) {
        let mut rounds = Vec::new();
        for line in input.lines() {
            rounds.push(line.parse()?);
        }
        Ok(Self { rounds })
    }

    part1 {
        Ok(self.rounds.iter().map(|r| r.response.vs(r.opponent)).sum())
    }

    part2 {
        Ok(self.rounds.iter().map(|r| r.response.vs2(r.opponent)).sum())
    }

    input = INPUT;
    test day2(INPUT, 13268, 15508);
}

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn vs(self, other: Self) -> usize {
        const WIN: usize = 6;
        const DRAW: usize = 3;
        const LOSE: usize = 0;
        match (self, other) {
            (RPS::Rock, RPS::Rock) => 1 + DRAW,
            (RPS::Rock, RPS::Paper) => 1 + LOSE,
            (RPS::Rock, RPS::Scissors) => 1 + WIN,
            (RPS::Paper, RPS::Rock) => 2 + WIN,
            (RPS::Paper, RPS::Paper) => 2 + DRAW,
            (RPS::Paper, RPS::Scissors) => 2 + LOSE,
            (RPS::Scissors, RPS::Rock) => 3 + LOSE,
            (RPS::Scissors, RPS::Paper) => 3 + WIN,
            (RPS::Scissors, RPS::Scissors) => 3 + DRAW,
        }
    }

    fn vs2(self, other: Self) -> usize {
        match self {
            RPS::Rock => match other {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            RPS::Paper => other,
            RPS::Scissors => match other {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
        }
        .vs(other)
    }
}

struct Round {
    opponent: RPS,
    response: RPS,
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, response) = s.split_once(' ').ok_or("invalid format")?;
        let opponent = match opponent {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => return Err("invalid opponent move"),
        };
        let response = match response {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => return Err("invalid response move"),
        };
        Ok(Self { opponent, response })
    }
}
