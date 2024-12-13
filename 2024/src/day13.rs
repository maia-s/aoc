use str_block::str_block;

use crate::Input;

pub const INPUTS: &[Input] = &[
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
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
    prize_x: u32,
    prize_y: u32,
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
}

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    'claws: for claw in input.split("\n\n") {
        let claw = Claw::new(claw);
        // todo: math
        let mut nb = (claw.prize_x / claw.b_x).min(claw.prize_y / claw.b_y);
        let mut rx = claw.prize_x - claw.b_x * nb;
        let mut ry = claw.prize_y - claw.b_y * nb;
        nb += 1;
        while nb != 0 {
            nb -= 1;
            let na = (rx / claw.a_x).min(ry / claw.a_y);
            if rx == na * claw.a_x && ry == na * claw.a_y {
                sum += 3 * na + nb;
                continue 'claws;
            }
            rx += claw.b_x;
            ry += claw.b_y;
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    0
}
