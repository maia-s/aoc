const INPUT: &str = include_str!("day-10.input");

fn main() {
    let mut part_1 = 0_usize;

    for line in INPUT.lines() {
        let mut stack = Vec::new();
        for c in line.trim().chars() {
            let (expect, points) = match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                    continue;
                }
                ')' => ('(', 3),
                ']' => ('[', 57),
                '}' => ('{', 1197),
                '>' => ('<', 25137),
                _ => panic!("unexpected `{}`", c),
            };
            if let Some(popped) = stack.pop() {
                if popped != expect {
                    part_1 += points;
                    break;
                }
            }
        }
    }

    println!("part 1: {}", part_1);
}
