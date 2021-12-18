use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign},
    str::FromStr,
};

const INPUT: &str = include_str!("day-18.input");

#[derive(Clone)]
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
                    }
                    Half::Right(r, n) => {
                        assert!(depth >= r);
                        depth -= r;
                    }
                }
            }
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
                        assert!(depth >= r);
                        depth -= r;
                        if n > split_treshold {
                            self.split(i);
                            continue 'reduce;
                        }
                    }
                }
            }
            return;
        }
    }

    fn explode(&mut self, i: usize) {
        match self.0[i] {
            Half::Left(_, n) => {
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

    fn split(&mut self, i: usize) {
        match self.0[i] {
            Half::Left(l, n) => {
                let nl = n / 2;
                let nr = (n + 1) / 2;
                self.0[i] = Half::Left(l + 1, nl);
                self.0.insert(i + 1, Half::Right(1, nr));
            }
            Half::Right(r, n) => {
                let nl = n / 2;
                let nr = (n + 1) / 2;
                self.0[i] = Half::Right(r + 1, nr);
                self.0.insert(i, Half::Left(1, nl));
            }
        }
    }

    fn magnitude(&self) -> u32 {
        todo!()
    }
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

impl Add for Number {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl AddAssign<&Self> for Number {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, rhs: &Self) {
        self.0.extend(rhs.0.iter());
        let len = self.0.len();
        self.0[0].nest();
        self.0[len - 1].nest();
        self.reduce();
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        let mut sum = iter.next().unwrap();
        for next in iter {
            sum += next;
        }
        sum
    }
}

#[derive(Clone, Copy)]
enum Half {
    Left(u32, u32),
    Right(u32, u32),
}

impl Half {
    fn nest(&mut self) -> u32 {
        match self {
            Half::Left(n, _) | Half::Right(n, _) => {
                *n += 1;
                *n
            }
        }
    }

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
    let input: Vec<_> = INPUT
        .lines()
        .map(|line| line.parse::<Number>().unwrap())
        .collect();
    let sum = input.iter().cloned().sum::<Number>();
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

    #[test]
    fn add() {
        let a = "[[[[4,3],4],4],[7,[[8,4],9]]]".parse::<Number>().unwrap();
        assert_eq!(a.to_string(), "[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = "[1,1]".parse::<Number>().unwrap();
        assert_eq!(b.to_string(), "[1,1]");
        let sum = a + b;
        assert_eq!(sum.to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn sum() {
        let n: Vec<Number> = vec![
            "[1,1]".parse().unwrap(),
            "[2,2]".parse().unwrap(),
            "[3,3]".parse().unwrap(),
            "[4,4]".parse().unwrap(),
        ];
        assert_eq!(
            n.into_iter().sum::<Number>().to_string(),
            "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );

        let n: Vec<Number> = vec![
            "[1,1]".parse().unwrap(),
            "[2,2]".parse().unwrap(),
            "[3,3]".parse().unwrap(),
            "[4,4]".parse().unwrap(),
            "[5,5]".parse().unwrap(),
        ];
        assert_eq!(
            n.into_iter().sum::<Number>().to_string(),
            "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );

        let n: Vec<Number> = vec![
            "[1,1]".parse().unwrap(),
            "[2,2]".parse().unwrap(),
            "[3,3]".parse().unwrap(),
            "[4,4]".parse().unwrap(),
            "[5,5]".parse().unwrap(),
            "[6,6]".parse().unwrap(),
        ];
        assert_eq!(
            n.into_iter().sum::<Number>().to_string(),
            "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );

        let n: Vec<Number> = vec![
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".parse().unwrap(),
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".parse().unwrap(),
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".parse().unwrap(),
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
                .parse()
                .unwrap(),
            "[7,[5,[[3,8],[1,4]]]]".parse().unwrap(),
            "[[2,[2,2]],[8,[8,1]]]".parse().unwrap(),
            "[2,9]".parse().unwrap(),
            "[1,[[[9,3],9],[[9,0],[0,7]]]]".parse().unwrap(),
            "[[[5,[7,4]],7],1]".parse().unwrap(),
            "[[[[4,2],2],6],[8,7]]".parse().unwrap(),
        ];

        assert_eq!(
            n[..2].iter().cloned().sum::<Number>().to_string(),
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );

        assert_eq!(
            n.into_iter().sum::<Number>().to_string(),
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
    }

    #[test]
    fn magnitude() {
        let n = "[9,1]".parse::<Number>().unwrap();
        assert_eq!(n.magnitude(), 29);

        let n = "[1,9]".parse::<Number>().unwrap();
        assert_eq!(n.magnitude(), 21);

        let n = "[[9,1],[1,9]]".parse::<Number>().unwrap();
        assert_eq!(n.magnitude(), 129);

        let n = "[[1,2],[[3,4],5]]".parse::<Number>().unwrap();
        assert_eq!(n.magnitude(), 143);

        let n = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
            .parse::<Number>()
            .unwrap();
        assert_eq!(n.magnitude(), 1384);

        let n = "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse::<Number>().unwrap();
        assert_eq!(n.magnitude(), 445);

        let n = "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse::<Number>().unwrap();
        assert_eq!(n.magnitude(), 791);

        let n = "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse::<Number>().unwrap();
        assert_eq!(n.magnitude(), 1137);

        let n = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse::<Number>()
            .unwrap();
        assert_eq!(n.magnitude(), 3488);
    }

    #[test]
    fn example() {
        let n: Vec<Number> = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
                .parse::<Number>()
                .unwrap(),
            "[[[5,[2,8]],4],[5,[[9,9],0]]]".parse::<Number>().unwrap(),
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"
                .parse::<Number>()
                .unwrap(),
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"
                .parse::<Number>()
                .unwrap(),
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"
                .parse::<Number>()
                .unwrap(),
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"
                .parse::<Number>()
                .unwrap(),
            "[[[[5,4],[7,7]],8],[[8,3],8]]".parse::<Number>().unwrap(),
            "[[9,3],[[9,9],[6,[4,9]]]]".parse::<Number>().unwrap(),
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"
                .parse::<Number>()
                .unwrap(),
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
                .parse::<Number>()
                .unwrap(),
        ];
        let sum = n.into_iter().sum::<Number>();
        assert_eq!(sum.magnitude(), 4140);
    }
}
