use std::str::FromStr;

const INPUT: &str = include_str!("day-12.input");

fn main() {
    let instructions: Vec<_> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
    println!("part 1: {}", part_1(&instructions));
}

#[derive(Copy, Clone)]
enum Instruction {
    NorthSouth(isize),
    WestEast(isize),
    LeftRight(isize),
    Forward(isize),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: isize = s[1..].parse().unwrap();
        match &s[0..=0] {
            "N" => Ok(Instruction::NorthSouth(-n)),
            "S" => Ok(Instruction::NorthSouth(n)),
            "W" => Ok(Instruction::WestEast(-n)),
            "E" => Ok(Instruction::WestEast(n)),
            "L" => Ok(Instruction::LeftRight(-n)),
            "R" => Ok(Instruction::LeftRight(n)),
            "F" => Ok(Instruction::Forward(n)),
            _ => panic!("unknown instruction: {}", s),
        }
    }
}

struct Ship {
    x: isize,
    y: isize,
    degrees: isize,
}

impl Ship {
    fn new() -> Self {
        Ship {
            x: 0,
            y: 0,
            degrees: 90,
        }
    }

    fn step(&mut self, i: Instruction) {
        match i {
            Instruction::NorthSouth(n) => self.y += n,
            Instruction::WestEast(n) => self.x += n,
            Instruction::LeftRight(n) => self.degrees = (self.degrees + n).rem_euclid(360),
            Instruction::Forward(n) => match self.degrees {
                0 => self.y -= n,
                90 => self.x += n,
                180 => self.y += n,
                270 => self.x -= n,
                _ => panic!("facing {}", self.degrees),
            },
        }
    }
}

fn part_1(instructions: &[Instruction]) -> isize {
    let mut ship = Ship::new();
    for &i in instructions.iter() {
        ship.step(i);
    }
    ship.x.abs() + ship.y.abs()
}
