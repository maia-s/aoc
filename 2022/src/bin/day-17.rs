use std::{error::Error, str::FromStr, iter, collections::VecDeque};

const INPUT: &str = include_str!("day-17.txt");

const INPUT_EX: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

aoc_2022::aoc! {
    struct Day17 {
        input: Input,
    }

    self(input) {
        Ok(Self { input: input.parse()? })
    }

    part1 usize {
        todo!()
    }

    part2 usize {
        todo!()
    }

    input = INPUT_EX;
}

#[derive(Clone, Copy)]
enum Move {
    Left,
    Right,
}

struct Input(Vec<Move>);

impl Input {
    fn iter(&self) -> impl Iterator<Item = Move> + '_ {
        self.0.iter().copied().cycle()
    }
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .as_bytes()
                .iter()
                .map(|b| match b {
                    b'<' => Ok(Move::Left),
                    b'>' => Ok(Move::Right),
                    _ => Err("invalid move"),
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Clone, Copy)]
enum Piece {
    HLine,
    Plus,
    J,
    VLine,
    Square,
}

impl Piece {
    fn generator() -> impl Iterator<Item = Piece> {
        let mut current = Piece::Square;
        iter::from_fn(move || {
            current = match current {
                Piece::HLine => Piece::Plus,
                Piece::Plus => Piece::J,
                Piece::J => Piece::VLine,
                Piece::VLine => Piece::Square,
                Piece::Square => Piece::HLine
            };
            Some(current)
        })
    }
}

struct Pit {
    rows: Vec<u8>,
    offset: usize,
}
