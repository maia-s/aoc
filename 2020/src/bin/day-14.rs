use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("day-14.input");

fn main() {
    let program: Vec<Instruction> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
    println!("part 1: {}", part_1(&program));
    println!("part 2: {}", part_2(&program));
}

#[derive(Copy, Clone)]
enum Instruction {
    Mask(u64, u64),
    Set(u64, u64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(" = ");
        let target = it.next().unwrap();
        let arg = it.next().unwrap();
        if target == "mask" {
            return Ok(Instruction::Mask(
                u64::from_str_radix(&arg.replace('X', "1"), 2).unwrap(),
                u64::from_str_radix(&arg.replace('X', "0"), 2).unwrap(),
            ));
        }
        if target.starts_with("mem[") && target.ends_with("]") {
            return Ok(Instruction::Set(
                target[4..(target.len() - 1)].parse().unwrap(),
                arg.parse().unwrap(),
            ));
        }
        panic!("illegal instruction: {}", s);
    }
}

fn part_1(program: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut and = 0xffff_ffff_ffff;
    let mut or = 0;
    let mut sum = 0;

    for &i in program.iter() {
        match i {
            Instruction::Mask(a, o) => {
                and = a;
                or = o;
            }

            Instruction::Set(addr, value) => {
                let value = (value & and) | or;
                sum += value;
                if let Some(prev) = mem.insert(addr, value) {
                    sum -= prev;
                }
            }
        }
    }

    sum
}

fn part_2(program: &[Instruction]) -> u64 {
    let mut mem = HashMap::new();
    let mut or = 0;
    let mut floats = vec![0];
    let mut sum = 0;

    for &i in program.iter() {
        match i {
            Instruction::Mask(a, o) => {
                or = o;

                let mut bits = !(!a | o) & 0xffff_ffff_ffff;
                floats.clear();
                floats.push(bits);
                loop {
                    let tz = bits.trailing_zeros();
                    if tz == 64 {
                        break;
                    }

                    let flip = 1 << tz;
                    bits ^= flip;

                    let n = floats.len();
                    floats.resize(n * 2, 0);
                    for i in 0..n {
                        floats[n + i] = floats[i] ^ flip;
                    }
                }
            }

            Instruction::Set(addr, value) => {
                for &f in floats.iter() {
                    let addr = (addr | or) ^ f;
                    sum += value;
                    if let Some(prev) = mem.insert(addr, value) {
                        sum -= prev;
                    }
                }
            }
        }
    }

    sum
}
