const INPUT: &str = include_str!("day-8.txt");

#[cfg(test)]
const INPUT_EX: &str = "30373
25512
65332
33549
35390";

aoc_2022::aoc! {
    struct Day8<'a> {
        trees: Vec<&'a [u8]>,
        maxx: usize,
        maxy: usize,
    }

    self(input) {
        let trees: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
        let maxy = trees.len() - 1;
        let maxx = trees[0].len() - 1;
        Ok(Self { trees, maxx, maxy })
    }

    part1 usize {
        Ok((0..=self.maxy).map(|y| (0..=self.maxx).map(|x| self.is_visible(x, y) as usize).sum::<usize>()).sum())
    }

    part2 usize {
        Ok((0..=self.maxy).map(|y| (0..=self.maxx).map(|x| self.scenic_score(x, y)).max().unwrap()).max().unwrap())
    }

    input = INPUT;
    test day8_ex(INPUT_EX, 21, 8);
    test day8(INPUT, 1676, 313200);
}

impl<'a> Day8<'a> {
    fn is_visible(&self, x: usize, y: usize) -> bool {
        if x == 0 || y == 0 || x == self.maxx || y == self.maxy {
            return true;
        }
        let tree = self.trees[y][x];
        let mut blocked: usize = 0;
        for i in (0..x).rev() {
            if self.trees[y][i] >= tree {
                blocked += 1;
                break;
            }
        }
        for i in (x + 1)..=self.maxx {
            if self.trees[y][i] >= tree {
                blocked += 1;
                break;
            }
        }
        for j in (0..y).rev() {
            if self.trees[j][x] >= tree {
                blocked += 1;
                break;
            }
        }
        for j in (y + 1)..=self.maxy {
            if self.trees[j][x] >= tree {
                blocked += 1;
                break;
            }
        }
        return blocked != 4;
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        if x == 0 || y == 0 || x == self.maxx || y == self.maxy {
            return 0;
        }
        let tree = self.trees[y][x];
        let mut left = 0;
        let mut right = 0;
        let mut up = 0;
        let mut down = 0;
        for i in (0..x).rev() {
            left += 1;
            if self.trees[y][i] >= tree {
                break;
            }
        }
        for i in (x + 1)..=self.maxx {
            right += 1;
            if self.trees[y][i] >= tree {
                break;
            }
        }
        for j in (0..y).rev() {
            up += 1;
            if self.trees[j][x] >= tree {
                break;
            }
        }
        for j in (y + 1)..=self.maxy {
            down += 1;
            if self.trees[j][x] >= tree {
                break;
            }
        }
        left * right * up * down
    }
}
