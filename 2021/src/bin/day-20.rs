use std::{collections::HashSet, fmt::Display, str::FromStr};

const INPUT: &str = include_str!("day-20.input");

struct Image {
    pixels: HashSet<(isize, isize)>,
}

impl Image {
    fn len(&self) -> usize {
        self.pixels.len()
    }

    fn get(&self, x: isize, y: isize) -> bool {
        self.pixels.contains(&(x, y))
    }

    fn chunk(&self, x: isize, y: isize) -> usize {
        let mut chunk = self.get(x - 1, y - 1) as usize;
        chunk = (chunk << 1) + self.get(x, y - 1) as usize;
        chunk = (chunk << 1) + self.get(x + 1, y - 1) as usize;
        chunk = (chunk << 1) + self.get(x - 1, y) as usize;
        chunk = (chunk << 1) + self.get(x, y) as usize;
        chunk = (chunk << 1) + self.get(x + 1, y) as usize;
        chunk = (chunk << 1) + self.get(x - 1, y + 1) as usize;
        chunk = (chunk << 1) + self.get(x, y + 1) as usize;
        chunk = (chunk << 1) + self.get(x + 1, y + 1) as usize;
        chunk
    }

    fn enhance(&mut self, lut: &[bool]) {
        self.pixels = self
            .pixels
            .iter()
            .filter_map(|&(x, y)| {
                if lut[self.chunk(x, y)] {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect();
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    pixels.insert((x as isize, y as isize));
                }
            }
        }
        Ok(Self { pixels })
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut min_x, mut max_x) = (isize::MAX, isize::MIN);
        let (mut min_y, mut max_y) = (isize::MAX, isize::MIN);
        for &(x, y) in self.pixels.iter() {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                write!(f, "{}", if self.get(x, y) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let (lut, mut image) = parse(INPUT);

    image.enhance(&lut);
    image.enhance(&lut);
    println!("part 1: {}", image.len());
}

fn parse(input: &str) -> (Vec<bool>, Image) {
    let (lut, image) = input.trim().split_once("\n\n").unwrap();
    let lut: Vec<_> = lut.trim().chars().map(|c| c == '#').collect();
    let image: Image = image.trim().parse().unwrap();
    (lut, image)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let (lut, mut image) = parse(input);
        assert!(lut[34]);
        eprintln!("{}", image);
        image.enhance(&lut);
        eprintln!("{}", image);
        image.enhance(&lut);
        eprintln!("{}", image);
        assert_eq!(image.len(), 35);
    }
}
