use std::{
    fmt::{self, Display},
    str::FromStr,
};

const INPUT: &str = "215694783";
const STEPS: usize = 100;

fn main() {
    let mut cups: Cups = INPUT.parse().unwrap();

    for _ in 0..STEPS {
        cups.step();
    }
    println!("part 1: {}", cups);
}

#[derive(Debug)]
struct Cups {
    cups: Ring<u8>,
    max: u8,
}

impl Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.cups.iter_from(&1.into());
        it.next();
        while let Some(item) = it.next() {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl FromStr for Cups {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut max = 0;
        let cups = Ring {
            vec: s
                .chars()
                .map(|c| {
                    let value = c as u8 - b'0';
                    max = max.max(value);
                    value
                })
                .collect(),
            cursor: 0,
            bookmark: 0,
        };
        Ok(Self { cups, max })
    }
}

impl Cups {
    fn step(&mut self) {
        self.cups.bookmark_current();
        let items = self.cups.take_clockwise(3);
        let mut label = *self.cups.current();
        loop {
            label = (label + self.max) % (self.max + 1);
            if self.cups.find(&label) {
                break;
            }
        }
        self.cups.insert_clockwise(items);
        self.cups.go_to_bookmark();
        self.cups.clockwise();
    }
}

#[derive(Debug)]
struct Ring<T> {
    vec: Vec<T>,
    cursor: usize,
    bookmark: usize,
}

impl<T> Ring<T> {
    fn len(&self) -> usize {
        self.vec.len()
    }

    fn current(&self) -> &T {
        &self.vec[self.cursor]
    }

    fn bookmark_current(&mut self) {
        self.bookmark = self.cursor;
    }

    fn go_to_bookmark(&mut self) {
        self.cursor = self.bookmark;
    }

    fn clockwise(&mut self) {
        self.cursor = (self.cursor + 1) % self.len()
    }

    fn counterclockwise(&mut self) {
        self.cursor = (self.cursor + self.len() - 1) % self.len()
    }

    fn find(&mut self, find: &T) -> bool
    where
        T: Eq + std::fmt::Display,
    {
        for (i, item) in self.vec.iter().enumerate() {
            if item == find {
                self.cursor = i;
                return true;
            }
        }
        false
    }

    fn take_clockwise(&mut self, n: usize) -> Vec<T> {
        (0..n)
            .map(|_| {
                self.clockwise();
                let item = self.vec.remove(self.cursor);
                if self.cursor < self.bookmark {
                    self.bookmark = (self.bookmark + self.len() - 1) % self.len();
                }
                self.counterclockwise();
                item
            })
            .collect()
    }

    fn insert_clockwise(&mut self, items: impl IntoIterator<Item = T>) {
        for (i, item) in items.into_iter().enumerate() {
            let insert_pos = self.cursor + i + 1;
            self.vec.insert(insert_pos, item);
            if insert_pos <= self.bookmark {
                self.bookmark = (self.bookmark + 1) % self.len();
            }
        }
    }

    fn iter_from(&self, from: &T) -> impl Iterator<Item = &T>
    where
        T: Eq,
    {
        let (second, first) = self
            .vec
            .split_at(self.vec.iter().position(|i| *i == *from).unwrap());
        first.iter().chain(second.iter())
    }
}
