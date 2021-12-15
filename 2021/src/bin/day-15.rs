use std::{cmp::Reverse, collections::BinaryHeap, str::FromStr};

const INPUT: &str = include_str!("day-15.input");

struct Map {
    map: Vec<u8>,
    width: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        Self {
            map: vec![0; width * height],
            width,
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.map.len() / self.width
    }

    fn path(&self, (fx, fy): (usize, usize), (tx, ty): (usize, usize)) -> usize {
        let mut visited = vec![false; self.map.len()];
        let mut path = BinaryHeap::from([Reverse(Node {
            cost: 0,
            x: fx,
            y: fy,
        })]);

        while let Some(Reverse(Node { cost, x, y })) = path.pop() {
            if !visited[y * self.width + x] {
                visited[y * self.width + x] = true;

                if (x, y) == (tx, ty) {
                    return cost;
                }

                let mut push = |x, y| {
                    path.push(Reverse(Node {
                        cost: cost + self.map[y * self.width + x] as usize,
                        x,
                        y,
                    }));
                };
                if y > 0 {
                    push(x, y - 1);
                }
                if x > 0 {
                    push(x - 1, y);
                }
                if x < self.width - 1 {
                    push(x + 1, y);
                }
                if y < self.height() - 1 {
                    push(x, y + 1);
                }
            }
        }

        panic!("no path");
    }

    fn blit(&mut self, (dx, dy): (usize, usize), src: &Map, add: u8) {
        for y in 0..src.height() {
            for x in 0..src.width() {
                self.map[(dy + y) * self.width + (dx + x)] =
                    (src.map[y * src.width() + x] - 1 + add) % 9 + 1;
            }
        }
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let map = s
            .lines()
            .flat_map(|line| {
                let line: Vec<_> = line.chars().map(|c| c as u8 - b'0').collect();
                if let Some(w) = width {
                    assert_eq!(line.len(), w);
                } else {
                    width = Some(line.len());
                }
                line
            })
            .collect();
        Ok(Self {
            map,
            width: width.unwrap(),
        })
    }
}

struct Node {
    cost: usize,
    x: usize,
    y: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

fn main() {
    let map: Map = INPUT.trim().parse().unwrap();

    println!(
        "part 1: {}",
        map.path((0, 0), (map.width() - 1, map.height() - 1))
    );

    let mut map2 = Map::new(map.width() * 5, map.height() * 5);
    for j in 0..5 {
        for i in 0..5 {
            map2.blit((i * map.width(), j * map.height()), &map, i as u8 + j as u8);
        }
    }

    println!(
        "part 2: {}",
        map2.path((0, 0), (map2.width() - 1, map2.height() - 1))
    );
}
