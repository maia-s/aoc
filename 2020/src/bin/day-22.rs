use std::{cmp::Ordering, collections::VecDeque, str::FromStr};

const INPUT: &str = include_str!("day-22.input");

fn main() {
    let game: Game = INPUT.parse().unwrap();

    println!("part 1: {}", part_1(game.clone()));
}

fn part_1(mut game: Game) -> usize {
    loop {
        if let Some(deck) = game.round() {
            break deck;
        }
    }
    .iter()
    .rev()
    .enumerate()
    .map(|(i, card)| (i + 1) * card)
    .sum()
}

#[derive(Clone)]
struct Game(Vec<Deck>);

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim()
                .split("\n\n")
                .map(|deck| deck.parse().unwrap())
                .collect(),
        ))
    }
}

impl Game {
    fn deck(&mut self, index: usize) -> &mut Deck {
        &mut self.0[index]
    }

    fn round(&mut self) -> Option<&Deck> {
        let drawn = [self.deck(0).draw(), self.deck(1).draw()];
        let winner = match drawn[0].cmp(&drawn[1]) {
            Ordering::Greater => 0,
            Ordering::Less => 1,
            Ordering::Equal => panic!("it's a draw!"),
        };
        let loser = 1 - winner;
        self.deck(winner).add_2(drawn[winner], drawn[loser]);
        if self.deck(loser).is_empty() {
            Some(self.deck(winner))
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Deck {
    name: String,
    cards: VecDeque<usize>,
}

impl FromStr for Deck {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines();
        let name = it.next().unwrap();
        let name = name[..name.rfind(':').unwrap()].to_string();
        let cards = it.map(|line| line.parse().unwrap()).collect();
        Ok(Self { name, cards })
    }
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn draw(&mut self) -> usize {
        self.cards.pop_front().unwrap()
    }

    fn add_2(&mut self, a: usize, b: usize) {
        self.cards.push_back(a);
        self.cards.push_back(b);
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.cards.iter().copied()
    }
}
