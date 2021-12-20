use std::{collections::HashMap, fmt::Display, str::FromStr};

const INPUT: &str = include_str!("day-20.input");

#[derive(Default)]
struct Image {
    pixels: HashMap<(isize, isize), bool>,
    unset: bool,
}

impl Image {
    fn lit(&self) -> usize {
        self.pixels.iter().filter(|(_, &set)| set).count()
    }

    fn get(&self, x: isize, y: isize) -> bool {
        *self.pixels.get(&(x, y)).unwrap_or(&self.unset)
    }

    fn set(&mut self, x: isize, y: isize, value: bool) {
        self.pixels.insert((x, y), value);
        self.pixels.entry((x - 1, y - 1)).or_insert(self.unset);
        self.pixels.entry((x, y - 1)).or_insert(self.unset);
        self.pixels.entry((x + 1, y - 1)).or_insert(self.unset);
        self.pixels.entry((x - 1, y)).or_insert(self.unset);
        self.pixels.entry((x + 1, y)).or_insert(self.unset);
        self.pixels.entry((x - 1, y + 1)).or_insert(self.unset);
        self.pixels.entry((x, y + 1)).or_insert(self.unset);
        self.pixels.entry((x + 1, y + 1)).or_insert(self.unset);
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
        let mut new = Self {
            unset: lut[self.unset as usize * 0x1ff],
            ..Default::default()
        };
        for (&(x, y), _) in self.pixels.iter() {
            new.set(x, y, lut[self.chunk(x, y)]);
        }
        *self = new;
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut image = Image::default();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                image.set(x as isize, y as isize, c == '#');
            }
        }
        Ok(image)
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut min_x, mut max_x) = (isize::MAX, isize::MIN);
        let (mut min_y, mut max_y) = (isize::MAX, isize::MIN);
        for (&(x, y), _) in self.pixels.iter() {
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
    println!("part 1: {}", image.lit());
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
        assert_eq!(image.chunk(2, 2), 34);
        image.enhance(&lut);
        eprintln!("{}", image);
        assert!(image.get(2, 2));
        image.enhance(&lut);
        eprintln!("{}", image);
        assert_eq!(image.lit(), 35);
    }
}
