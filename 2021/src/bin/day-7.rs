const INPUT: &str = include_str!("day-7.input");

fn main() {
    let input: Vec<_> = INPUT
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut i = input.iter().sum::<usize>() / input.len();
    let mut min_i = i;
    let mut min_cost = cost(&input, i);
    let d = if i > 0 { -1 } else { 1 };

    while (0..input.len()).contains(&((i as isize + d) as usize)) {
        i = (i as isize + d) as usize;
        let cost = cost(&input, i);
        if min_cost > cost {
            min_cost = cost;
            min_i = i;
        } else {
            break;
        }
    }
    println!("part 1: {}", min_cost);
}

fn cost(crabs: &[usize], pos: usize) -> usize {
    crabs
        .iter()
        .map(|&x| if x > pos { x - pos } else { pos - x })
        .sum()
}
