use crate::Input;
use str_block::str_block;

pub const INPUTS: &[Input<i64>] = &[
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
];

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
    pub fn new(input: &str) -> Self {
        let (buttons, prize) = input.split_once("\nPrize: X=").unwrap();
        let (button_a, button_b) = buttons.split_once("\nButton B: X+").unwrap();
        let button_a = button_a.strip_prefix("Button A: X+").unwrap();
        let (a_x, a_y) = button_a.split_once(", Y+").unwrap();
        let (b_x, b_y) = button_b.split_once(", Y+").unwrap();
        let (prize_x, prize_y) = prize.split_once(", Y=").unwrap();
        Self {
            a_x: a_x.parse().unwrap(),
            a_y: a_y.parse().unwrap(),
            b_x: b_x.parse().unwrap(),
            b_y: b_y.parse().unwrap(),
            prize_x: prize_x.parse().unwrap(),
            prize_y: prize_y.trim_ascii_end().parse().unwrap(),
        }
    }

    pub fn calc(&self) -> i64 {
        let nb = (self.prize_y * self.a_x - self.prize_x * self.a_y)
            / (self.b_y * self.a_x - self.b_x * self.a_y);
        let rx = self.prize_x - nb * self.b_x;
        let ry = self.prize_y - nb * self.b_y;
        let na = (rx / self.a_x).min(ry / self.a_y);
        (rx == na * self.a_x && ry == na * self.a_y) as i64 * (3 * na + nb)
    }
}

pub fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for claw in input.split("\n\n") {
        let claw = Claw::new(claw);
        sum += claw.calc();
    }
    sum
}

pub fn part2(input: &str) -> i64 {
    let mut sum = 0;
    for claw in input.split("\n\n") {
        let mut claw = Claw::new(claw);
        claw.prize_x += 10000000000000;
        claw.prize_y += 10000000000000;
        sum += claw.calc();
    }
    sum
}
