use std::str::FromStr;

const INPUT: &str = include_str!("day-2.input");

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_ascii_whitespace();
        let cmd = s.next().unwrap();
        let n = s.next().unwrap().parse::<usize>().unwrap();
        Ok(match cmd {
            "forward" => Command::Forward(n),
            "down" => Command::Down(n),
            "up" => Command::Up(n),
            _ => panic!("unknown command `{}`", cmd),
        })
    }
}

fn main() {
    let input: Vec<_> = INPUT.lines().map(|s| s.parse::<Command>().unwrap()).collect();

    // part 1
    let mut xpos = 0;
    let mut depth = 0;
    for cmd in input.iter() {
        match cmd {
            Command::Forward(n) => xpos += n,
            Command::Down(n) => depth += n,
            Command::Up(n) => depth -= n,
        }
    }
    println!("part 1: {}", xpos * depth);
}
