use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day-4.input");

struct Board {
    cells: HashMap<usize, Square>,
    row_marks: [usize; 5],
    col_marks: [usize; 5],
    won: bool,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, n) in line.split_ascii_whitespace().enumerate() {
                let value = n.parse().unwrap();
                cells.insert(
                    value,
                    Square {
                        x,
                        y,
                        marked: false,
                    },
                );
            }
        }
        Ok(Self {
            cells,
            row_marks: [0; 5],
            col_marks: [0; 5],
            won: false,
        })
    }
}

impl Board {
    fn mark(&mut self, value: usize) -> bool {
        if !self.won {
            if let Some(cell) = self.cells.get_mut(&value) {
                cell.marked = true;
                self.row_marks[cell.y] += 1;
                self.col_marks[cell.x] += 1;
                self.won = self.row_marks[cell.y] == 5 || self.col_marks[cell.x] == 5;
                self.won
            } else {
                false
            }
        } else {
            false
        }
    }

    fn unmarked_sum(&self) -> usize {
        self.cells
            .iter()
            .filter_map(|(value, square)| if square.marked { None } else { Some(value) })
            .sum()
    }
}

struct Square {
    x: usize,
    y: usize,
    marked: bool,
}

fn main() {
    let mut input = INPUT.split("\n\n");
    let numbers: Vec<usize> = input
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut boards: Vec<Board> = input.map(|s| s.parse().unwrap()).collect();

    let mut part_1 = true;
    let mut last_score = 0;

    for &i in numbers.iter() {
        for board in boards.iter_mut() {
            if board.mark(i) {
                let score = board.unmarked_sum() * i;
                last_score = score;

                if part_1 {
                    part_1 = false;
                    println!("part 1: {}", score);
                }
            }
        }
    }

    println!("part 2: {}", last_score);
}
