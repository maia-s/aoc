use std::str::FromStr;

const INPUT: &str = include_str!("day-8.input");

enum Op {
    Acc(isize),
    Jmp(isize),
    Nop,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let instr = it.next().unwrap();
        let arg = it.next().unwrap().parse().unwrap();
        match instr {
            "acc" => Ok(Op::Acc(arg)),
            "jmp" => Ok(Op::Jmp(arg)),
            "nop" => Ok(Op::Nop),
            _ => panic!("illegal instruction: {}", instr),
        }
    }
}

struct VM {
    ops: Vec<(bool, Op)>,
    ip: isize,
    acc: isize,
}

impl FromStr for VM {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vm = VM {
            ops: vec![],
            ip: 0,
            acc: 0,
        };
        for line in s.lines() {
            vm.ops.push((false, line.parse().unwrap()));
        }
        Ok(vm)
    }
}

impl VM {
    fn step(&mut self) -> bool {
        let ip = self.ip as usize;
        match self.ops[ip] {
            (false, Op::Acc(x)) => {
                self.ops[ip].0 = true;
                self.acc += x;
                self.ip += 1
            }
            (false, Op::Jmp(x)) => {
                self.ops[ip].0 = true;
                self.ip += x
            }
            (false, Op::Nop) => {
                self.ops[ip].0 = true;
                self.ip += 1
            }
            (true, _) => return false,
        }
        return true;
    }
}

fn main() {
    println!("part 1: {}", part_1());
}

fn part_1() -> isize {
    let mut vm: VM = INPUT.parse().unwrap();
    while vm.step() {}
    vm.acc
}
