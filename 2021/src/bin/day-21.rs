use std::str::FromStr;

const INPUT: &str = include_str!("day-21.input");

enum Universe<D: Die, const WIN: u16> {
    Game(Game<D, WIN>),
    Won(u8, usize),
}

#[derive(Clone)]
struct Game<D: Die, const WIN: u16> {
    die: D,
    players: Vec<Player>,
    current_player: u8,
    current_roll: u16,
    roll_count: u16,
}

impl<D: Die, const WIN: u16> FromStr for Game<D, WIN> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let players: Vec<_> = s
            .trim()
            .lines()
            .map(|line| {
                let (_, start) = line.split_once(':').unwrap();
                Player {
                    pos: start.trim().parse().unwrap(),
                    score: 0,
                }
            })
            .collect();
        assert_eq!(players.len(), 2);
        Ok(Self {
            die: D::default(),
            players,
            current_player: 0,
            current_roll: 0,
            roll_count: 0,
        })
    }
}

impl<D: Die, const WIN: u16> Game<D, WIN> {
    fn roll(mut self) -> Vec<Universe<D, WIN>> {
        self.die
            .roll()
            .into_iter()
            .map(|roll| {
                let mut game = self.clone();
                game.current_roll += roll as u16;
                game.roll_count += 1;
                if game.roll_count % 3 == 0 {
                    let current_player = game.current_player;
                    let score = game.players[current_player as usize].forward(game.current_roll);
                    game.current_player = (game.current_player + 1) % game.players.len() as u8;
                    game.current_roll = 0;
                    if score >= WIN {
                        Universe::Won(
                            current_player,
                            game.players[game.current_player as usize].score as usize
                                * game.roll_count as usize,
                        )
                    } else {
                        Universe::Game(game)
                    }
                } else {
                    Universe::Game(game)
                }
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
struct Player {
    pos: u8,
    score: u16,
}

impl Player {
    fn forward(&mut self, n: u16) -> u16 {
        self.pos = ((self.pos as u16 + n - 1) % 10 + 1) as u8;
        self.score += self.pos as u16;
        self.score
    }
}

trait Die: Copy + Default {
    fn roll(&mut self) -> Vec<u8>;
}

#[derive(Clone, Copy, Default)]
struct DeterministicDie(u8);

impl Die for DeterministicDie {
    fn roll(&mut self) -> Vec<u8> {
        let roll = self.0 + 1;
        if roll == 100 {
            self.0 = 0;
        } else {
            self.0 = roll;
        }
        vec![roll]
    }
}

#[derive(Clone, Copy, Default)]
struct DiracDie;

impl Die for DiracDie {
    fn roll(&mut self) -> Vec<u8> {
        vec![1, 2, 3]
    }
}

fn main() {
    let part_1 = play::<DeterministicDie, 1000>(INPUT, true);
    println!("part 1: {}", part_1);

    let part_2 = play::<DiracDie, 21>(INPUT, false);
    println!("part 2: {}", part_2);
}

fn play<D: Die, const WIN: u16>(input: &str, get_first_win: bool) -> usize {
    let start = input.parse::<Game<D, WIN>>().unwrap();
    let mut wins = vec![0; start.players.len()];
    let mut multiverse = vec![Universe::Game(start)];
    while !multiverse.is_empty() {
        match multiverse.pop().unwrap() {
            Universe::Game(game) => multiverse.extend(game.roll()),
            Universe::Won(player, score) => {
                if get_first_win {
                    return score;
                }
                wins[player as usize] += 1;
            }
        }
    }
    wins.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";
        assert_eq!(play::<DeterministicDie, 1000>(input, true), 739785);
    }
}
