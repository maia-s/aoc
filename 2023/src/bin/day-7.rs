use std::{cmp::Ordering, collections::HashMap, fmt::Debug, str::FromStr};

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-7.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"};

aoc! {
    struct Day7<'a> {
        input: &'a str,
    }

    self(input = INPUT) {
        Ok(Self { input })
    }

    1 part1 usize {
        let mut hands = self.input.lines().map(|line| {
            let (hand, bid) = line.split_once(' ').ok_or("missing space")?;
            Ok((hand.parse()?, bid.parse().map_err(|_| "parse error")?))
        }).collect::<Result<Vec<(Hand, usize)>, Error>>()?;
        hands.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        Ok(hands.iter().enumerate().map(|(i, hand)| (i + 1) * hand.1).sum())
    }

    2 part2 usize {
        let input = self.input.replace('J', "j");
        Day7 { input: &input }.part1()
    }

    INPUT_EX { 1 part1 = 6440, 2 part2 = 5905 }
    INPUT { 1 part1 = 246409899, 2 part2 = 244848487 }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand([Card; 5]);

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_a: Type = (*self).into();
        let type_b: Type = (*other).into();
        let type_cmp = type_a.cmp(&type_b);
        if !type_cmp.is_eq() {
            type_cmp
        } else if let Some(cmp) = self
            .0
            .into_iter()
            .zip(other.0)
            .map(|(a, b)| a.cmp(&b))
            .find(|c| !c.is_eq())
        {
            cmp
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(Card::new)
                .collect::<Result<Vec<Card>, _>>()?
                .try_into()
                .map_err(|_| "hand size wasn't five cards")?,
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::FiveOfAKind => "Five of a kind",
                Self::FourOfAKind => "Four of a kind",
                Self::FullHouse => "Full house",
                Self::ThreeOfAKind => "Three of a kind",
                Self::TwoPairs => "Two pairs",
                Self::OnePair => "One pair",
                Self::HighCard => "High card",
            }
        )
    }
}

impl From<Hand> for Type {
    fn from(value: Hand) -> Self {
        let mut counts = HashMap::new();
        for card in value.0 {
            *counts.entry(card).or_default() += 1;
        }
        let jokers = counts.get(&Card::Joker).copied().unwrap_or(0);
        let mut counts: Vec<usize> = counts.values().copied().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        match counts.len() {
            1 => Self::FiveOfAKind,
            2 => match (jokers, counts[0], counts[1]) {
                (4, 4, 1) => Self::FiveOfAKind,
                (1, 4, 1) => Self::FiveOfAKind,
                (0, 4, 1) => Self::FourOfAKind,
                (3, 3, 2) => Self::FiveOfAKind,
                (2, 3, 2) => Self::FiveOfAKind,
                (0, 3, 2) => Self::FullHouse,
                _ => unreachable!(),
            },
            3 => match (jokers, counts[0], counts[1]) {
                (3, 3, 1) => Self::FourOfAKind,
                (1, 3, 1) => Self::FourOfAKind,
                (0, 3, 1) => Self::ThreeOfAKind,
                (2, 2, 2) => Self::FourOfAKind,
                (1, 2, 2) => Self::FullHouse,
                (0, 2, 2) => Self::TwoPairs,
                _ => unreachable!(),
            },
            4 => match jokers {
                2 => Self::ThreeOfAKind,
                1 => Self::ThreeOfAKind,
                0 => Self::OnePair,
                _ => unreachable!(),
            },
            5 => match jokers {
                1 => Self::OnePair,
                0 => Self::HighCard,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn new(card: char) -> Result<Self, Error> {
        match card {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            'j' => Ok(Self::Joker),
            s => Err(format!("invalid card: {s}").into()),
        }
    }
}
