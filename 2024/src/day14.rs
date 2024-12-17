use core::cmp::Ordering;

use crate::Input;
//use str_block::str_block;

pub fn inputs() -> Vec<Input> {
    vec![
        Input::Hashed("5034a7e89989b8e93bae7b3b718406144bcaf278c692a774a3c7acc8e3b6e56d"),
        /*Input::Inline( // size 11,7
            "example",
            str_block! {"
                p=0,4 v=3,-3
                p=6,3 v=-1,-3
                p=10,3 v=-1,2
                p=2,0 v=2,-1
                p=0,0 v=1,3
                p=3,0 v=-2,-2
                p=7,6 v=-1,-3
                p=3,0 v=-1,-2
                p=9,3 v=2,3
                p=7,3 v=-1,2
                p=2,4 v=2,-3
                p=9,5 v=-3,-3
            "},
            Some(12),
            None,
        ),*/
    ]
}

const SPACE_W: i64 = 101;
const SPACE_H: i64 = 103;

const MAX_X: i64 = i64::MAX - i64::MAX % SPACE_W;
const MAX_Y: i64 = i64::MAX - i64::MAX % SPACE_H;

fn num<const SIGNED: bool>(input: &[u8], mut i: usize) -> (i32, usize) {
    let mut m = 1;
    if SIGNED && input[i] == b'-' {
        i += 1;
        m = -1;
    }
    let mut num = (input[i] - b'0') as i32;
    while {
        i += 1;
        input[i] >= b'0'
    } {
        num = num * 10 + (input[i] - b'0') as i32;
    }
    (num * m, i)
}

struct Robot {
    px: i64,
    py: i64,
    vx: i32,
    vy: i32,
}

impl Robot {
    #[inline(always)]
    fn new(input: &[u8], i: &mut usize) -> Option<Self> {
        if *i >= input.len() {
            return None;
        }
        let (px, j) = num::<false>(input, *i);
        let (py, j) = num::<false>(input, j + 1);
        let (vx, j) = num::<true>(input, j + 3);
        let (vy, j) = num::<true>(input, j + 1);
        *i = j + 3;
        Some(Self {
            px: px as i64,
            py: py as i64,
            vx,
            vy,
        })
    }

    fn step(&mut self, n: i32) {
        let nx = self.px + n as i64 * self.vx as i64;
        let ny = self.py + n as i64 * self.vy as i64;
        let nx = if nx >= 0 { nx } else { MAX_X + nx };
        let ny = if ny >= 0 { ny } else { MAX_Y + ny };
        self.px = (nx as u64 % SPACE_W as u64) as i64;
        self.py = (ny as u64 % SPACE_H as u64) as i64;
    }

    fn quadrant(&self) -> usize {
        match (self.px.cmp(&(SPACE_W / 2)), self.py.cmp(&(SPACE_H / 2))) {
            (Ordering::Less, Ordering::Less) => 1,
            (Ordering::Greater, Ordering::Less) => 2,
            (Ordering::Less, Ordering::Greater) => 3,
            (Ordering::Greater, Ordering::Greater) => 4,
            _ => 0,
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut i = 2;
    let mut q = [0; 5];
    while let Some(mut robot) = Robot::new(input, &mut i) {
        robot.step(100);
        q[robot.quadrant()] += 1;
    }
    q[1] * q[2] * q[3] * q[4]
}

pub fn part2(_input: &str) -> u32 {
    0
}
