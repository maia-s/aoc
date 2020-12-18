use std::iter::Peekable;

const INPUT: &str = include_str!("day-18.input");

fn main() {
    let input: Vec<Peekable<Lexer>> = INPUT
        .lines()
        .map(|line| {
            let lexer: Lexer = line.into();
            lexer.peekable()
        })
        .collect();
    println!("part 1: {}", part_1(input.clone()));
}

fn part_1(input: Vec<Peekable<Lexer>>) -> usize {
    fn parse(lexer: &mut Peekable<Lexer>) -> usize {
        let mut value = match lexer.next() {
            Some(Token::Lit(n)) => n,
            Some(Token::ParenL) => parse(lexer),
            x => panic!("unexpected {:?}", x),
        };

        loop {
            match lexer.next() {
                Some(op @ Token::Add) | Some(op @ Token::Mul) => {
                    let rhs = match lexer.next() {
                        Some(Token::Lit(rhs)) => rhs,
                        Some(Token::ParenL) => parse(lexer),
                        x => panic!("unexpected {:?}", x),
                    };
                    value = match op {
                        Token::Add => value + rhs,
                        Token::Mul => value * rhs,
                        _ => unreachable!(),
                    };
                }
                Some(Token::ParenR) | None => break value,
                x => panic!("unexpected {:?}", x),
            }
        }
    }
    input.into_iter().map(|mut expr| parse(&mut expr)).sum()
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Lit(usize),
    Add,
    Mul,
    ParenL,
    ParenR,
}

#[derive(Copy, Clone)]
struct Lexer<'a> {
    input: &'a [u8],
    index: usize,
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(s: &'a str) -> Self {
        Self {
            input: s.as_bytes(),
            index: 0,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index;
            self.index += 1;
            match self.input.get(index) {
                Some(b' ') => (),
                Some(b'+') => break Some(Token::Add),
                Some(b'*') => break Some(Token::Mul),
                Some(b'(') => break Some(Token::ParenL),
                Some(b')') => break Some(Token::ParenR),
                Some(&n @ b'0'..=b'9') => break Some(Token::Lit((n - b'0') as usize)),
                Some(x) => panic!("unexpected '{}'", x),
                None => break None,
            }
        }
    }
}
