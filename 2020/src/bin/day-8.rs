use std::str::FromStr;

const INPUT: &str = include_str!("day-8.input");

enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
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
            "nop" => Ok(Op::Nop(arg)),
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
    fn step(&mut self) -> Option<bool> {
        let ip = self.ip as usize;
        match self.ops[ip] {
            (false, Op::Acc(x)) => self.acc += x,
            (false, Op::Jmp(x)) => self.ip += x - 1,
            (false, Op::Nop(_)) => {}
            (true, _) => return Some(false),
        }
        self.ops[ip].0 = true;
        self.ip += 1;
        if self.ip as usize >= self.ops.len() {
            Some(true)
        } else {
            None
        }
    }

    fn run(&mut self) -> bool {
        loop {
            match self.step() {
                Some(x) => return x,
                None => (),
            }
        }
    }
}

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> isize {
    let mut vm: VM = INPUT.parse().unwrap();
    vm.run();
    vm.acc
}

fn part_2() -> isize {
    let mut i = 0;
    loop {
        let mut vm: VM = INPUT.parse().unwrap();
        loop {
            match vm.ops[i].1 {
                Op::Acc(_) => i += 1,
                Op::Jmp(x) => { vm.ops[i].1 = Op::Nop(x); break; }
                Op::Nop(x) => { vm.ops[i].1 = Op::Jmp(x); break; }
            }
        }
        if vm.run() {
            return vm.acc;
        }
        i += 1;
    }
}
