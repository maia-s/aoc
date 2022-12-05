use std::{error::Error, fmt::Display, str::FromStr};

const INPUT: &str = include_str!("day-5.txt");

#[cfg(test)]
const INPUT_EX: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

aoc_2022::aoc! {
    #[derive(Clone, Debug)]
    struct Day5 {
        stacks: Vec<Vec<char>>,
        commands: Vec<Command>,
    }

    self(input) {
        let Some((stack, cmds)) = input.split_once("\n\n") else {
            return Err("invalid input".into());
        };
        let mut stacks = Vec::new();
        'parse: for (_, s) in stack.lines().enumerate() {
            for (j, b) in s.as_bytes().chunks(4).enumerate() {
                if stacks.len() <= j {
                    stacks.resize(j + 1, Vec::new());
                }
                match (b[0], b[1], b[2]) {
                    (b'[', n, b']') => stacks[j].insert(0, char::from(n)),
                    (b' ', b' ', b' ') => continue,
                    (b' ', _, b' ') => break 'parse,
                    (_, _, _) => return Err("invalid box".into()),
                }
            }
        }
        let commands = cmds.lines().map(|i| i.parse()).collect::<Result<_,_>>()?;
        Ok(Self { stacks, commands })
    }

    part1 String {
        let mut p1 = self.clone();
        for &Command { mov, from, to } in p1.commands.iter() {
            for _ in 0..mov {
                let b = p1.stacks[from - 1].pop().unwrap();
                p1.stacks[to - 1].push(b);
            }
        }
        Ok(p1.to_string())
    }

    part2 String {
        for &Command { mov, from, to } in self.commands.iter() {
            let ip = self.stacks[to - 1].len();
            for _ in 0..mov {
                let b = self.stacks[from - 1].pop().unwrap();
                self.stacks[to - 1].insert(ip, b);
            }
        }
        Ok(self.to_string())
    }

    input = INPUT;
    test day5_ex(INPUT_EX, "CMZ", "MCD");
    test day5(INPUT, "QPJPLMNNR", "BQDNWJPVJ");
}

impl Display for Day5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.stacks.iter() {
            write!(f, "{}", i.last().unwrap())?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct Command {
    mov: usize,
    from: usize,
    to: usize,
}

impl FromStr for Command {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("move ") else { return Err("missing `move`".into()) };
        let Some((mov, s)) = s.split_once(" from ") else { return Err("missing `from`".into()) };
        let Some((from, to)) = s.split_once(" to ") else { return Err("missing `to`".into()) };
        let (mov, from, to) = (mov.parse()?, from.parse()?, to.parse()?);
        Ok(Self { mov, from, to })
    }
}
