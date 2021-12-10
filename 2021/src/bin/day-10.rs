const INPUT: &str = include_str!("day-10.input");

fn main() {
    let mut part_1 = 0_usize;
    let mut part_2 = Vec::new();
    let mut stack = Vec::new();

    for line in INPUT.lines() {
        stack.clear();
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
                    stack.clear();
                    break;
                }
            }
        }
        if !stack.is_empty() {
            let mut score = 0_usize;
            while let Some(popped) = stack.pop() {
                score = score * 5
                    + match popped {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => unreachable!(),
                    };
            }
            part_2.push(score);
        }
    }

    println!("part 1: {}", part_1);

    part_2.sort_unstable();

    println!("part 2: {}", part_2[part_2.len() / 2]);
}
