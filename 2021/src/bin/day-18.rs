use std::{fmt::Display, ops::AddAssign, str::FromStr};

const INPUT: &str = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";

struct Number(Vec<Half>);

impl Number {
    fn reduce(&mut self) {
        let explode_depth = 4;
        let split_treshold = 9;
        'reduce: loop {
            let mut depth = 0;
            for (i, &h) in self.0.iter().enumerate() {
                match h {
                    Half::Left(l, n) => {
                        depth += l;
                        if depth > explode_depth {
                            self.explode(i);
                            continue 'reduce;
                        }
                        if n > split_treshold {
                            self.split(i);
                            continue 'reduce;
                        }
                    }
                    Half::Right(r, n) => {
                        depth -= r;
                        if n > split_treshold {
                            self.split(i);
                            continue 'reduce;
                        }
                    }
                }
            }
            break;
        }
    }

    fn explode(&mut self, i: usize) {
        match self.0[i] {
            Half::Left(l, n) => {
                assert!(i < self.0.len() - 1);
                assert!(matches!(self.0[i + 1], Half::Right(_, _)));
                if i > 0 {
                    self.0[i - 1] += n;
                }
                if i < self.0.len() - 2 {
                    let r = self.0[i + 1];
                    self.0[i + 2] += r;
                }
            }
            Half::Right(_, _) => unreachable!(),
        }
        if self.0[i].denest() >= self.0[i + 1].denest() {
            self.0[i].set(0);
            self.0.remove(i + 1);
        } else {
            self.0[i + 1].set(0);
            self.0.remove(i);
        }
    }

    fn split(&mut self, i: usize) {}
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        for n in s.split(',') {
            let mut n = n;
            if n.starts_with('[') {
                let mut l = 0;
                while n.starts_with('[') {
                    n = &n[1..];
                    l += 1;
                }
                v.push(Half::Left(l, n.parse().unwrap()));
            } else {
                let mut r = 0;
                while n.ends_with(']') {
                    n = &n[..n.len() - 1];
                    r += 1;
                }
                v.push(Half::Right(r, n.parse().unwrap()));
            }
        }
        Ok(Self(v))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for &h in self.0.iter() {
            if !first {
                write!(f, ",")?;
            }
            first = false;
            match h {
                Half::Left(l, n) => {
                    for _ in 0..l {
                        write!(f, "[")?;
                    }
                    write!(f, "{}", n)?;
                }
                Half::Right(r, n) => {
                    write!(f, "{}", n)?;
                    for _ in 0..r {
                        write!(f, "]")?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Half {
    Left(u32, u32),
    Right(u32, u32),
}

impl Half {
    fn denest(&mut self) -> u32 {
        match self {
            Half::Left(n, _) | Half::Right(n, _) => {
                *n -= 1;
                *n
            }
        }
    }

    fn set(&mut self, n: u32) {
        *self = match self {
            Half::Left(i, _) => Half::Left(*i, n),
            Half::Right(i, _) => Half::Right(*i, n),
        };
    }
}

impl AddAssign for Half {
    fn add_assign(&mut self, rhs: Half) {
        match rhs {
            Half::Left(_, n) | Half::Right(_, n) => *self += n,
        }
    }
}

impl AddAssign<u32> for Half {
    fn add_assign(&mut self, rhs: u32) {
        match self {
            Half::Left(_, n) | Half::Right(_, n) => *n += rhs,
        }
    }
}

fn main() {
    let a = INPUT.parse::<Number>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explode() {
        let mut n = "[[[[[9,8],1],2],3],4]".parse::<Number>().unwrap();
        assert_eq!(n.to_string(), "[[[[[9,8],1],2],3],4]");
        n.reduce();
        assert_eq!(n.to_string(), "[[[[0,9],2],3],4]");

        let mut n = "[7,[6,[5,[4,[3,2]]]]]".parse::<Number>().unwrap();
        assert_eq!(n.to_string(), "[7,[6,[5,[4,[3,2]]]]]");
        n.reduce();
        assert_eq!(n.to_string(), "[7,[6,[5,[7,0]]]]");

        let mut n = "[[6,[5,[4,[3,2]]]],1]".parse::<Number>().unwrap();
        assert_eq!(n.to_string(), "[[6,[5,[4,[3,2]]]],1]");
        n.reduce();
        assert_eq!(n.to_string(), "[[6,[5,[7,0]]],3]");

        let mut n = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<Number>()
            .unwrap();
        assert_eq!(n.to_string(), "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        n.reduce();
        assert_eq!(n.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");

        let mut n = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<Number>()
            .unwrap();
        assert_eq!(n.to_string(), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        n.reduce();
        assert_eq!(n.to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }
}
