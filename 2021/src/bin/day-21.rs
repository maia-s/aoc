use std::str::FromStr;

const INPUT: &str = include_str!("day-21.input");

struct Game<D: Die> {
    die: D,
    players: Vec<Player>,
    roll_count: usize,
}

impl<D: Die> FromStr for Game<D> {
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
        Ok(Self {
            die: D::default(),
            players,
            roll_count: 0,
        })
    }
}

impl<D: Die> Game<D> {
    fn play(&mut self) -> usize {
        loop {
            if let Some(score) = self.round() {
                return score;
            }
        }
    }

    fn round(&mut self) -> Option<usize> {
        assert_eq!(self.players.len(), 2);
        for (i, p) in self.players.iter_mut().enumerate() {
            let roll = self.die.roll_n(3);
            self.roll_count += 3;
            if p.forward(roll) {
                return Some(self.players[1 - i].score * self.roll_count);
            }
        }
        None
    }
}

struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn forward(&mut self, n: usize) -> bool {
        self.pos = (self.pos + n - 1) % 10 + 1;
        self.score += self.pos;
        self.score >= 1000
    }
}

trait Die: Default {
    fn roll(&mut self) -> usize;

    fn roll_n(&mut self, n: usize) -> usize {
        let mut sum = 0;
        for _ in 0..n {
            sum += self.roll();
        }
        sum
    }
}

#[derive(Default)]
struct DeterministicDie(usize);

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        let roll = self.0 + 1;
        if roll == 100 {
            self.0 = 0;
        } else {
            self.0 = roll;
        }
        roll
    }
}

fn main() {
    let mut game = INPUT.parse::<Game<DeterministicDie>>().unwrap();
    println!("part 1: {}", game.play());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Player 1 starting position: 4\nPlayer 2 starting position: 8";        
        let mut game = input.parse::<Game<DeterministicDie>>().unwrap();
        assert_eq!(game.play(), 739785);
    }
}
