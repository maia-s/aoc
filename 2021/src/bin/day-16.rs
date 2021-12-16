use std::{str::FromStr, vec};

const INPUT: &str = include_str!("day-16.input");

struct Transmission {
    bits: vec::IntoIter<bool>,
    pos: usize,
    version_sum: usize,
}

impl Transmission {
    fn read_n(&mut self, mut n: usize) -> usize {
        let mut value = 0;
        self.pos += n;
        while n > 0 {
            n -= 1;
            value = (value << 1) + self.bits.next().unwrap() as usize;
        }
        value
    }

    fn packet(&mut self) -> usize {
        let version = self.read_n(3);
        self.version_sum += version;
        match self.read_n(3) {
            4 => self.literal(),
            x => self.operator(x),
        }
    }

    fn literal(&mut self) -> usize {
        let mut value = 0;
        while self.read_n(1) == 1 {
            value = (value << 4) + self.read_n(4);
        }
        (value << 4) + self.read_n(4)
    }

    fn operator(&mut self, typeid: usize) -> usize {
        let mut subpackets = Vec::new();
        if self.read_n(1) == 0 {
            let length = self.read_n(15);
            let i = self.pos;
            while self.pos - i < length {
                subpackets.push(self.packet());
            }
            assert_eq!(self.pos - i, length);
        } else {
            let count = self.read_n(11);
            for _ in 0..count {
                subpackets.push(self.packet());
            }
        }
        let mut it = subpackets.into_iter();
        match typeid {
            0 => it.sum(),
            1 => it.product(),
            2 => it.min().unwrap(),
            3 => it.max().unwrap(),
            5 => {
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                (a > b) as usize
            }
            6 => {
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                (a < b) as usize
            }
            7 => {
                let a = it.next().unwrap();
                let b = it.next().unwrap();
                (a == b) as usize
            }
            x => panic!("operator packet type {}", x),
        }
    }
}

impl FromStr for Transmission {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bits: s
                .trim()
                .chars()
                .flat_map(|c| match c {
                    '0' => b"0000",
                    '1' => b"0001",
                    '2' => b"0010",
                    '3' => b"0011",
                    '4' => b"0100",
                    '5' => b"0101",
                    '6' => b"0110",
                    '7' => b"0111",
                    '8' => b"1000",
                    '9' => b"1001",
                    'A' => b"1010",
                    'B' => b"1011",
                    'C' => b"1100",
                    'D' => b"1101",
                    'E' => b"1110",
                    'F' => b"1111",
                    _ => panic!("unexpected `{}`", c),
                })
                .map(|&b| b != b'0')
                .collect::<Vec<_>>()
                .into_iter(),
            pos: 0,
            version_sum: 0,
        })
    }
}

fn main() {
    let mut transmission = INPUT.parse::<Transmission>().unwrap();
    let part_2 = transmission.packet();
    println!("part 1: {}", transmission.version_sum);
    println!("part 2: {}", part_2);
}
