use std::{cmp::Ordering, collections::BinaryHeap};

use aoc_2023::{aoc, str_block, Error};

const INPUT: &str = include_str!("day-17.txt");

#[allow(dead_code)]
const INPUT_EX: &str = str_block! {"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"};

#[allow(dead_code)]
const INPUT_EX2: &str = str_block! {"
111111111111
999999999991
999999999991
999999999991
999999999991
"};

aoc! {
    struct Day17 {
        map: Vec<Vec<u8>>,
    }

    self(input = INPUT) {
        Ok(Self { map: input.lines().map(
            |line| line.chars().map(|c| {
                assert!(c.is_ascii_digit());
                (c as u32 - '0' as u32) as u8
            }).collect()
        ).collect() })
    }

    1 part1 usize {
        Ok(self.pathfind::<0, 3>()?)
    }

    2 part2 usize {
        Ok(self.pathfind::<4, 10>()?)
    }

    INPUT_EX { 1 part1 = 102, 2 part2 = 94 }
    INPUT_EX2 { 2 part2 = 71 }
    INPUT { 1 part1 = 1065, 2 part2 = 1249 }
}

impl Day17 {
    fn pathfind<const MIN_STRAIGHT: usize, const MAX_STRAIGHT: usize>(
        &self,
    ) -> Result<usize, Error> {
        let mut visited: Vec<Vec<[[usize; 4]; MAX_STRAIGHT]>> = self
            .map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|_| [[usize::MAX; 4]; MAX_STRAIGHT])
                    .collect()
            })
            .collect();
        let width = visited[0].len() as i32;
        let height = visited.len() as i32;

        #[derive(PartialEq, Eq)]
        struct Node {
            cost: usize,
            x: i32,
            y: i32,
            run: [u8; 4],
            entered_dir: Dir,
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                let cost_cmp = other.cost.cmp(&self.cost);
                if cost_cmp != Ordering::Equal {
                    cost_cmp
                } else {
                    (self.x, self.y).cmp(&(other.x, other.y))
                }
            }
        }
        let mut queue = BinaryHeap::new();
        queue.push(Node {
            cost: 0,
            x: 0,
            y: 0,
            run: [0; 4],
            entered_dir: Dir::N,
        });

        let mut first = true;

        while let Some(Node {
            cost,
            x,
            y,
            run,
            entered_dir,
        }) = queue.pop()
        {
            if x == width - 1 && y == height - 1 {
                return Ok(cost);
            }
            let mut push = |x, y, dir: Dir| {
                if (0..width).contains(&x)
                    && (0..height).contains(&y)
                    && run[dir as usize] < MAX_STRAIGHT as u8
                    && (first
                        || (entered_dir as usize + 2) % 4 != dir as usize
                            && (dir == entered_dir
                                || run[entered_dir as usize] >= MIN_STRAIGHT as u8))
                {
                    let mut new_run = run;
                    if dir != entered_dir {
                        if !match dir {
                            Dir::N => y >= MIN_STRAIGHT as i32,
                            Dir::E => x <= width - MIN_STRAIGHT as i32,
                            Dir::S => y <= height - MIN_STRAIGHT as i32,
                            Dir::W => x >= MIN_STRAIGHT as i32,
                        } {
                            return;
                        }
                        new_run[entered_dir as usize] = 0;
                    }
                    let ndr = new_run[dir as usize] as usize;
                    new_run[dir as usize] += 1;
                    let new_cost = cost + self.map[y as usize][x as usize] as usize;
                    if visited[y as usize][x as usize][ndr][dir as usize] > new_cost {
                        visited[y as usize][x as usize][ndr][dir as usize] = new_cost;
                        queue.push(Node {
                            cost: new_cost,
                            x,
                            y,
                            run: new_run,
                            entered_dir: dir,
                        });
                    }
                }
            };
            push(x, y - 1, Dir::N);
            push(x - 1, y, Dir::W);
            push(x + 1, y, Dir::E);
            push(x, y + 1, Dir::S);
            first = false;
        }

        Err("path not found".into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Dir {
    N,
    E,
    S,
    W,
}
