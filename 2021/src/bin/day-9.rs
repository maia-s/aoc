use std::str::FromStr;

const INPUT: &str = include_str!("day-9.input");

struct Map {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: isize, y: isize) -> u8 {
        if (0..self.width).contains(&(x as usize)) && (0..self.height).contains(&(y as usize)) {
            self.grid[y as usize * self.width + x as usize]
        } else {
            10
        }
    }

    fn for_each(&self, mut f: impl FnMut(u8, u8, u8, u8, u8)) {
        for y in 0..self.height {
            let y = y as isize;
            for x in 0..self.width {
                let x = x as isize;
                f(
                    self.get(x, y),
                    self.get(x, y - 1),
                    self.get(x + 1, y),
                    self.get(x, y + 1),
                    self.get(x - 1, y),
                );
            }
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let grid: Vec<u8> = s
            .lines()
            .flat_map(|line| {
                let line = line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>();
                if let Some(width) = width {
                    assert_eq!(line.len(), width);
                } else {
                    width = Some(line.len());
                }
                line
            })
            .collect();
        let width = width.unwrap();
        let height = grid.len() / width;
        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

fn main() {
    let map: Map = INPUT.parse().unwrap();

    let mut risk = 0;
    map.for_each(|c, n, e, s, w| {
        if c < n && c < e && c < s && c < w {
            risk += c as usize + 1;
        }
    });
    println!("part 1: {}", risk);
}
