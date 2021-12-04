use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day-4.input");

struct Board {
    cells: HashMap<usize, Square>,
    row_marks: [usize; 5],
    col_marks: [usize; 5],
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
                        value,
                        marked: false,
                    },
                );
            }
        }
        Ok(Self {
            cells,
            row_marks: [0; 5],
            col_marks: [0; 5],
        })
    }
}

impl Board {
    fn mark(&mut self, value: usize) -> bool {
        if let Some(cell) = self.cells.get_mut(&value) {
            cell.marked = true;
            self.row_marks[cell.y] += 1;
            self.col_marks[cell.x] += 1;
            self.row_marks[cell.y] == 5 || self.col_marks[cell.x] == 5
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
    value: usize,
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

    println!("part 1: {}", part_1(&numbers, &mut boards));
}

fn part_1(numbers: &[usize], boards: &mut [Board]) -> usize {
    for &i in numbers.iter() {
        for board in boards.iter_mut() {
            if board.mark(i) {
                return board.unmarked_sum() * i;
            }
        }
    }
    panic!("no winners");
}
