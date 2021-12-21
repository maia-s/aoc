use std::str::FromStr;

const INPUT: &str = include_str!("day-21.input");

enum Universe<D: Die, const WIN: u16> {
    Game(Game<D, WIN>),
    Won(usize, u8, usize),
}

#[derive(Clone)]
struct Game<D: Die, const WIN: u16> {
    die: D,
    players: Vec<Player>,
    current_player: u8,
    roll_count: u16,
    weight: usize,
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
            roll_count: 0,
            weight: 1,
        })
    }
}

impl<D: Die, const WIN: u16> Game<D, WIN> {
    fn roll(mut self) -> Vec<Universe<D, WIN>> {
        self.die
            .roll_3()
            .into_iter()
            .map(|(weight, roll)| {
                let mut game = self.clone();
                game.weight *= weight;
                game.roll_count += 3;
                let current_player = game.current_player;
                let score = game.players[current_player as usize].forward(roll);
                game.current_player = (game.current_player + 1) % game.players.len() as u8;
                if score >= WIN {
                    Universe::Won(
                        game.weight,
                        current_player,
                        game.players[game.current_player as usize].score as usize
                            * game.roll_count as usize,
                    )
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
    fn forward(&mut self, n: u32) -> u16 {
        let pos = (self.pos as u32 + n - 1) % 10 + 1;
        self.pos = pos as u8;
        self.score += pos as u16;
        self.score
    }
}

trait Die: Copy + Default {
    fn roll_3(&mut self) -> Vec<(usize, u32)>;
}

#[derive(Clone, Copy, Default)]
struct DeterministicDie(u8);

impl DeterministicDie {
    fn roll(&mut self) -> u32 {
        let roll = self.0 + 1;
        if roll == 100 {
            self.0 = 0;
        } else {
            self.0 = roll;
        }
        roll as u32
    }
}

impl Die for DeterministicDie {
    fn roll_3(&mut self) -> Vec<(usize, u32)> {
        vec![(1, self.roll() + self.roll() + self.roll())]
    }
}

#[derive(Clone, Copy, Default)]
struct DiracDie;

impl Die for DiracDie {
    fn roll_3(&mut self) -> Vec<(usize, u32)> {
        vec![(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)]
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
            Universe::Won(weight, player, score) => {
                if get_first_win {
                    return score;
                }
                wins[player as usize] += weight;
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
