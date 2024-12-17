use crate::Input;
use core::mem::transmute;
use str_block::str_block;

pub fn inputs() -> Vec<Input<String>> {
    vec![
        Input::Hashed("b5525f8b9f107fb1466897b6c3167837c5dd9a604481f82a4270be03872d43c2"),
        Input::Inline(
            "example",
            str_block! {"
                Register A: 729
                Register B: 0
                Register C: 0

                Program: 0,1,5,4,3,0
            "},
            Some("4,6,3,5,6,3,5,2,1,0".into()),
            None,
        ),
    ]
}

fn num(input: &[u8], i: &mut usize) -> i32 {
    let mut j = *i;
    let mut num = (input[j] - b'0') as i32;
    loop {
        j += 1;
        let digit = input[j];
        if digit >= b'0' {
            num = num * 10 + (digit - b'0') as i32;
        } else {
            *i = j;
            return num;
        }
    }
}

trait Output {
    fn out(&mut self, num: u8);
}

impl Output for String {
    #[inline(always)]
    fn out(&mut self, num: u8) {
        if !self.is_empty() {
            self.push(',');
        }
        self.push(char::from(num + b'0'));
    }
}

#[derive(Debug)]
struct Program {
    a: i32,
    b: i32,
    c: i32,
    ip: usize,
    ram: Vec<u8>,
}

impl Program {
    fn new(input: &[u8]) -> Self {
        let mut i = 12;
        let a = num(input, &mut i);
        i += 13;
        let b = num(input, &mut i);
        i += 13;
        let c = num(input, &mut i);
        i += 11;
        let mut ram = Vec::with_capacity((input.len() - i) / 2);
        loop {
            ram.push(input[i] - b'0');
            i += 2;
            if i >= input.len() {
                break;
            }
        }
        Self {
            a,
            b,
            c,
            ip: 0,
            ram,
        }
    }

    fn combo(&self, arg: u8) -> i32 {
        match arg {
            0..4 => arg as _,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn dv(&self, arg: u8) -> i32 {
        self.a >> self.combo(arg)
    }

    fn step(&mut self, out: &mut impl Output) -> bool {
        if self.ip >= self.ram.len() {
            return false;
        }
        let op = unsafe { transmute::<u8, Op>(self.ram[self.ip]) };
        let arg = self.ram[self.ip + 1];
        self.ip += 2;
        match op {
            Op::Adv => self.a = self.dv(arg),
            Op::Bxl => self.b ^= arg as i32,
            Op::Bst => self.b = (self.combo(arg) & 7) as _,
            Op::Jnz => {
                if self.a != 0 {
                    self.ip = arg as _
                }
            }
            Op::Bxc => self.b ^= self.c,
            Op::Out => out.out((self.combo(arg) & 7) as u8),
            Op::Bdv => self.b = self.dv(arg),
            Op::Cdv => self.c = self.dv(arg),
        }
        true
    }
}

#[repr(u8)]
#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

pub fn part1(input: &str) -> String {
    let mut program = Program::new(input.as_bytes());
    let mut out = String::new();
    while program.step(&mut out) {}
    out
}

pub fn part2(_input: &str) -> String {
    String::new()
}
