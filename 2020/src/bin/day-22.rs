use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
    str::FromStr,
};

const INPUT: &str = include_str!("day-22.input");

fn main() {
    let game: Game = INPUT.parse().unwrap();

    println!("part 1: {}", part_1(game.clone()));
    println!("part 2: {}", part_2(game));
}

fn part_1(mut game: Game) -> usize {
    let deck = game.run_normal();
    game.deck(deck).score()
}

fn part_2(mut game: Game) -> usize {
    let deck = game.run_recursive();
    game.deck(deck).score()
}

#[derive(Clone)]
struct Game {
    decks: Vec<Deck>,
    hashes: HashSet<u64>,
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            decks: s
                .trim()
                .split("\n\n")
                .map(|deck| deck.parse().unwrap())
                .collect(),
            hashes: HashSet::new(),
        })
    }
}

impl Game {
    fn deck(&mut self, index: usize) -> &mut Deck {
        &mut self.decks[index]
    }

    fn run_normal(&mut self) -> usize {
        loop {
            if let Some(deck) = self.normal_round() {
                break deck;
            }
        }
    }

    fn run_recursive(&mut self) -> usize {
        loop {
            if let Some(deck) = self.recursive_round() {
                break deck;
            }
        }
    }

    fn normal_round(&mut self) -> Option<usize> {
        let drawn = [self.deck(0).draw(), self.deck(1).draw()];
        self.highest_win(&drawn)
    }

    fn recursive_round(&mut self) -> Option<usize> {
        let mut hasher = DefaultHasher::new();
        self.decks.hash(&mut hasher);
        if !self.hashes.insert(hasher.finish()) {
            return Some(0);
        }
        let drawn = [self.deck(0).draw(), self.deck(1).draw()];
        if self.deck(0).len() >= drawn[0] && self.deck(1).len() >= drawn[1] {
            let mut subgame = self.clone();
            subgame.deck(0).truncate(drawn[0]);
            subgame.deck(1).truncate(drawn[1]);
            self.round_end(&drawn, subgame.run_recursive())
        } else {
            self.highest_win(&drawn)
        }
    }

    fn highest_win(&mut self, drawn: &[usize; 2]) -> Option<usize> {
        self.round_end(
            drawn,
            match drawn[0].cmp(&drawn[1]) {
                Ordering::Greater => 0,
                Ordering::Less => 1,
                Ordering::Equal => panic!("it's a draw!"),
            },
        )
    }

    fn round_end(&mut self, drawn: &[usize; 2], winner: usize) -> Option<usize> {
        let loser = 1 - winner;
        self.deck(winner).push_2(drawn[winner], drawn[loser]);
        if self.deck(loser).is_empty() {
            Some(winner)
        } else {
            None
        }
    }
}

#[derive(Clone, Hash)]
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

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn truncate(&mut self, len: usize) {
        self.cards.truncate(len)
    }

    fn draw(&mut self) -> usize {
        self.cards.pop_front().unwrap()
    }

    fn push_2(&mut self, a: usize, b: usize) {
        self.cards.push_back(a);
        self.cards.push_back(b);
    }

    fn score(&self) -> usize {
        self.iter()
            .rev()
            .enumerate()
            .map(|(i, card)| (i + 1) * card)
            .sum()
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = usize> + '_ {
        self.cards.iter().copied()
    }
}
