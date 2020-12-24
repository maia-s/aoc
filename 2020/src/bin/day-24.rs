use std::{collections::HashSet, mem::swap};

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

    let mut set = HashSet::new();
    for line in input.iter() {
        let pos = line.iter().fold((0, 0), |pos, &dir| {
            let delta = dir.delta();
            (pos.0 + delta.0, pos.1 + delta.1)
        });
        if !set.insert(pos) {
            set.remove(&pos);
        }
    }

    println!("part 1: {}", set.len());

    for _ in 0..100 {
        step(&mut set);
    }

    println!("part 2: {}", set.len());
}

fn step(set: &mut HashSet<(isize, isize)>) {
    let mut next = HashSet::new();
    for &(x, y) in set.iter() {
        match neighbor_coords(x, y)
            .map(|(x, y)| {
                if neighbor_coords(x, y)
                    .map(|c| set.get(&c).is_some() as usize)
                    .sum::<usize>()
                    == 2
                {
                    next.insert((x, y));
                }
                set.get(&(x, y)).is_some() as usize
            })
            .sum()
        {
            1 | 2 => {
                next.insert((x, y));
            }
            _ => (),
        }
    }
    swap(set, &mut next);
}

fn neighbor_coords(x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> {
    [
        Direction::East,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
        Direction::NorthEast,
    ]
    .iter()
    .map(move |dir| {
        let delta = dir.delta();
        (x + delta.0, y + delta.1)
    })
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
