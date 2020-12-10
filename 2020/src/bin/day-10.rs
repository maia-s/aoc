const INPUT: &str = include_str!("day-10.input");

fn main() {
    let mut jolts: Vec<_> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
    jolts.push(0);
    jolts.sort_unstable();

    println!("part 1: {}", part_1(&jolts));
    println!("part 2: {}", part_2(&jolts));
}

fn part_1(jolts: &[usize]) -> usize {
    let mut ones = 0;
    let mut threes = 1;
    for i in jolts.windows(2) {
        match i[1] - i[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }
    ones * threes
}

fn part_2(jolts: &[usize]) -> usize {
    fn combinations(memo: &mut [Option<usize>], jolts: &[usize], index: usize) -> usize {
        match memo[index] {
            Some(n) => n,
            None => {
                let mut it = jolts[index..].iter();
                let base = it.next().unwrap();
                let mut combos = 0;
                let mut ci = 0;
                while let Some(i) = it.next() {
                    ci += 1;
                    if i - base <= 3 {
                        combos += combinations(memo, jolts, index + ci);
                    } else {
                        break;
                    }
                }
                if ci == 0 {
                    combos = 1;
                }
                memo[index] = Some(combos);
                combos
            }
        }
    }

    combinations(&mut vec![None; jolts.len()], jolts, 0)
}
