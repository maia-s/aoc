use std::str::FromStr;

const INPUT: &str = include_str!("day-5.input");

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

struct Grid {
    cells: Vec<usize>,
    width: usize,
}

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        Self {
            cells: vec![0; w * h],
            width: w,
        }
    }

    fn ortho_line(&mut self, from: &Point, to: &Point) {
        if from.x == to.x {
            let x = from.x;
            let from_y = from.y.min(to.y);
            let to_y = from.y.max(to.y);
            for y in from_y..=to_y {
                self.set(x, y);
            }
        } else if from.y == to.y {
            let y = from.y;
            let from_x = from.x.min(to.x);
            let to_x = from.x.max(to.x);
            for x in from_x..=to_x {
                self.set(x, y);
            }
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        self.cells[y * self.width + x] += 1;
    }

    fn count(&self, min: usize) -> usize {
        self.cells.iter().filter(move |&&c| c >= min).count()
    }
}

fn main() {
    let input: Vec<_> = INPUT
        .lines()
        .map(|s| {
            let (from, to) = s.split_once(" -> ").unwrap();
            (from.parse::<Point>().unwrap(), to.parse::<Point>().unwrap())
        })
        .collect();

    let mut max_x = 0;
    let mut max_y = 0;
    for (from, to) in input.iter() {
        max_x = max_x.max(from.x.max(to.x));
        max_y = max_y.max(from.y.max(to.y));
    }
    let mut grid = Grid::new(max_x + 1, max_y + 1);

    for (from, to) in input.iter() {
        grid.ortho_line(from, to);
    }

    println!("part 1: {}", grid.count(2));
}
