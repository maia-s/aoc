use aoc_2023::Error;
use std::str::FromStr;

const INPUT: &str = include_str!("day-2.txt");

#[allow(dead_code)]
const INPUT_EX: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

aoc_2023::aoc! {
    struct Day2 {
        games: Vec<Game>,
    }

    self(input) {
        Ok(Self { games: input.lines().map(|line| {
            let (_, line) = line.split_once(": ").ok_or_else(|| Error::from("invalid line"))?;
            line.parse::<Game>()
        }).collect::<Result<Vec<_>, _>>()?})
    }

    part1 usize {
        Ok(self.games.iter().enumerate().filter(|(_, game)| game.is_valid(12, 13, 14)).map(|(i, _)| i + 1).sum())
    }

    input = INPUT;

    test day2_example(INPUT_EX, 8);
}

struct Game {
    round: Vec<Round>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            round: s
                .split("; ")
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Game {
    fn is_valid(&self, r: usize, g: usize, b: usize) -> bool {
        self.round.iter().all(|round| round.is_valid(r, g, b))
    }
}

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for s in s.split(", ") {
            let (n, col) = s
                .split_once(' ')
                .ok_or_else(|| Error::from("missing space in round"))?;
            let n = n
                .parse()
                .map_err(|_| Error(format!("invalid quantity `{n}`")))?;
            match col {
                "red" => red = n,
                "green" => green = n,
                "blue" => blue = n,
                _ => return Err(Error::from("unknown color")),
            }
        }
        Ok(Self { red, green, blue })
    }
}

impl Round {
    fn is_valid(&self, r: usize, g: usize, b: usize) -> bool {
        self.red <= r && self.green <= g && self.blue <= b
    }
}
