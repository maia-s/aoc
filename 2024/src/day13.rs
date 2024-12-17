use crate::Input;
use str_block::str_block;

pub fn inputs() -> Vec<Input<i64>> {
    vec![
        Input::Hashed("718c37dfc74608dac8c5adf3e07a9ed14f983de89864811eabfc059a1210759c"),
        Input::Inline(
            "example",
            str_block! {"
                Button A: X+94, Y+34
                Button B: X+22, Y+67
                Prize: X=8400, Y=5400

                Button A: X+26, Y+66
                Button B: X+67, Y+21
                Prize: X=12748, Y=12176

                Button A: X+17, Y+86
                Button B: X+84, Y+37
                Prize: X=7870, Y=6450

                Button A: X+69, Y+23
                Button B: X+27, Y+71
                Prize: X=18641, Y=10279
            "},
            Some(480),
            None,
        ),
    ]
}

#[inline(always)]
fn num(input: &[u8], mut i: usize) -> (u32, usize) {
    let mut n = 0;
    while i < input.len() && input[i] >= b'0' {
        n = n * 10 + (input[i] - b'0') as u32;
        i += 1;
    }
    (n, i)
}

#[derive(Debug)]
struct Claw {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

impl Claw {
    #[inline(always)]
    pub fn new(input: &[u8], i: usize) -> (Self, usize) {
        let (a_x, i) = num(input, i + 12);
        let (a_y, i) = num(input, i + 4);
        let (b_x, i) = num(input, i + 13);
        let (b_y, i) = num(input, i + 4);
        let (prize_x, i) = num(input, i + 10);
        let (prize_y, i) = num(input, i + 4);
        (
            Self {
                a_x: a_x as _,
                a_y: a_y as _,
                b_x: b_x as _,
                b_y: b_y as _,
                prize_x: prize_x as _,
                prize_y: prize_y as _,
            },
            i + 2,
        )
    }

    #[inline(always)]
    pub fn calc(&self) -> i64 {
        let nb = (self.prize_y * self.a_x - self.prize_x * self.a_y)
            / (self.b_y * self.a_x - self.b_x * self.a_y);
        let rx = self.prize_x - nb * self.b_x;
        let ry = self.prize_y - nb * self.b_y;
        let na = rx / self.a_x;
        (rx == na * self.a_x && ry == na * self.a_y) as i64 * (3 * na + nb)
    }
}

pub fn part1(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut i = 0;
    let mut claw;
    while i < input.len() {
        (claw, i) = Claw::new(input, i);
        sum += claw.calc();
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut i = 0;
    let mut claw;
    while i < input.len() {
        (claw, i) = Claw::new(input, i);
        claw.prize_x += 10000000000000;
        claw.prize_y += 10000000000000;
        sum += claw.calc();
    }
    sum
}
