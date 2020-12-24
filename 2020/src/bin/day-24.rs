use std::collections::HashSet;

const INPUT: &str = include_str!("day-24.input");

fn main() {
    let input: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let mut it = line.chars();
            let mut directions = vec![];
            while let Some(ch) = it.next() {
                directions.push(match ch {
                    'e' => Direction::East,
                    'w' => Direction::West,
                    's' => match it.next().unwrap() {
                        'e' => Direction::SouthEast,
                        'w' => Direction::SouthWest,
                        _ => panic!(),
                    },
                    'n' => match it.next().unwrap() {
                        'e' => Direction::NorthEast,
                        'w' => Direction::NorthWest,
                        _ => panic!(),
                    },
                    _ => panic!(),
                });
            }
            directions
        })
        .collect();

    let mut map = HashSet::new();
    for line in input.iter() {
        let pos = line.iter().fold((0, 0), |pos, &dir| {
            let delta = dir.delta();
            (pos.0 + delta.0, pos.1 + delta.1)
        });
        if !map.insert(pos) {
            map.remove(&pos);
        }
    }

    println!("part 1: {}", map.len());
}

#[derive(Clone, Copy)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    /*
        .   .
       / \ / \
      |-1 | 1 |
      |-1 |-1 |
     / \ / \ / \
    |-2 | 0 | 2 |
    | 0 | 0 | 0 |
     \ / \ / \ /
      |-1 | 1 |
      | 1 | 1 |
       \ / \ /
        '   '
    */
    fn delta(self) -> (isize, isize) {
        match self {
            Direction::East => (2, 0),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-2, 0),
            Direction::NorthWest => (-1, -1),
            Direction::NorthEast => (1, -1),
        }
    }
}
