use core::fmt::Display;

use crate::Input;
use rustc_hash::FxHashMap;
use str_block::str_block;

pub fn inputs() -> Vec<Input<u64>> {
    vec![
        Input::Hashed("e8e55402a684c7dfd7a47896571602ebba1b2c07b06c0edb3ac2bc0683e979aa"),
        Input::Inline(
            "example",
            str_block! {"
                x00: 1
                x01: 1
                x02: 1
                y00: 0
                y01: 1
                y02: 0

                x00 AND y00 -> z00
                x01 XOR y01 -> z01
                x02 OR y02 -> z02
            "},
            Some(4),
            None,
        ),
        Input::Inline(
            "larger example",
            str_block! {"
                x00: 1
                x01: 0
                x02: 1
                x03: 1
                x04: 0
                y00: 1
                y01: 1
                y02: 1
                y03: 1
                y04: 1

                ntg XOR fgs -> mjb
                y02 OR x01 -> tnw
                kwq OR kpj -> z05
                x00 OR x03 -> fst
                tgd XOR rvg -> z01
                vdt OR tnw -> bfw
                bfw AND frj -> z10
                ffh OR nrd -> bqk
                y00 AND y03 -> djm
                y03 OR y00 -> psh
                bqk OR frj -> z08
                tnw OR fst -> frj
                gnj AND tgd -> z11
                bfw XOR mjb -> z00
                x03 OR x00 -> vdt
                gnj AND wpb -> z02
                x04 AND y00 -> kjc
                djm OR pbm -> qhw
                nrd AND vdt -> hwm
                kjc AND fst -> rvg
                y04 OR y02 -> fgs
                y01 AND x02 -> pbm
                ntg OR kjc -> kwq
                psh XOR fgs -> tgd
                qhw XOR tgd -> z09
                pbm OR djm -> kpj
                x03 XOR y03 -> ffh
                x00 XOR y04 -> ntg
                bfw OR bqk -> z06
                nrd XOR fgs -> wpb
                frj XOR qhw -> z04
                bqk OR frj -> z07
                y03 OR x01 -> nrd
                hwm AND bqk -> z03
                tgd XOR rvg -> z12
                tnw OR pbm -> gnj
            "},
            Some(2024),
            None,
        ),
    ]
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Wire([u8; 3], u8);

impl Wire {
    fn zindex(&self) -> u32 {
        self.1 as _
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            char::from(self.0[0]),
            char::from(self.0[1]),
            char::from(self.0[2])
        )
    }
}

impl From<&[u8]> for Wire {
    fn from(value: &[u8]) -> Self {
        let zindex = if value[0] == b'z' {
            (value[1] - b'0') * 10 + value[2] - b'0'
        } else {
            0
        };
        Self([value[0], value[1], value[2]], zindex)
    }
}

#[derive(Clone, Copy)]
enum Gate {
    And(Wire, Wire),
    Or(Wire, Wire),
    Xor(Wire, Wire),
    Bool(bool),
}

struct Gates {
    gates: FxHashMap<Wire, Gate>,
    zmax: u32,
}

impl Gates {
    fn new(input: &str) -> Self {
        let mut gates = FxHashMap::default();
        let (inputs, gatedefs) = input.split_once("\n\n").unwrap();
        let mut zmax = 0;
        for line in inputs.lines() {
            let (wire, value) = line.split_once(": ").unwrap();
            let wire = Wire::from(wire.as_bytes());
            let value = value == "1";
            zmax = zmax.max(wire.zindex());
            gates.insert(wire, Gate::Bool(value));
        }
        for line in gatedefs.lines() {
            let (op, wire) = line.split_once(" -> ").unwrap();
            let op = op.as_bytes();
            let lhs = op[..3].into();
            let gate = match op[4] {
                b'A' => {
                    let rhs = op[8..].into();
                    Gate::And(lhs, rhs)
                }
                b'O' => {
                    let rhs = op[7..].into();
                    Gate::Or(lhs, rhs)
                }
                b'X' => {
                    let rhs = op[8..].into();
                    Gate::Xor(lhs, rhs)
                }
                _ => unreachable!(),
            };
            let wire = Wire::from(wire.as_bytes());
            zmax = zmax.max(wire.zindex());
            gates.insert(wire, gate);
        }
        Self { gates, zmax }
    }

    fn get(&mut self, wire: Wire) -> bool {
        match self.gates.get(&wire).copied() {
            Some(Gate::And(lhs, rhs)) => {
                let value = self.get(lhs) && self.get(rhs);
                self.gates.insert(wire, Gate::Bool(value));
                value
            }
            Some(Gate::Or(lhs, rhs)) => {
                let value = self.get(lhs) || self.get(rhs);
                self.gates.insert(wire, Gate::Bool(value));
                value
            }
            Some(Gate::Xor(lhs, rhs)) => {
                let value = self.get(lhs) != self.get(rhs);
                self.gates.insert(wire, Gate::Bool(value));
                value
            }
            Some(Gate::Bool(value)) => value,
            None => false,
        }
    }

    fn z(&mut self) -> u64 {
        let mut wire = [b'z', 0, 0];
        let mut z = 0;
        for i in (0..self.zmax as u8 + 1).rev() {
            wire[1] = i / 10 + b'0';
            wire[2] = i % 10 + b'0';
            let bit = self.get(Wire(wire, i)) as u64;
            z = z << 1 | bit;
        }
        z
    }
}

pub fn part1(input: &str) -> u64 {
    Gates::new(input).z()
}

pub fn part2(_input: &str) -> u64 {
    0
}
