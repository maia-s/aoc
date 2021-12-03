const INPUT: &str = include_str!("day-3.input");

fn main() {
    let input: Vec<Vec<_>> = INPUT
        .lines()
        .map(|s| s.chars().map(|c| c as usize - b'0' as usize).collect())
        .collect();
    let digits = input[0].len();
    let n = input.len() as f64;

    let sums = input.iter().fold(vec![0; digits], |mut sum, v| {
        for (s, i) in sum.iter_mut().zip(v.iter()) {
            *s += i;
        }
        sum
    });

    let gamma = num(&sums
        .iter()
        .map(|&i| i as f64 / n >= 0.5)
        .collect::<Vec<bool>>());
    let epsilon = num(&sums
        .iter()
        .map(|&i| i as f64 / n < 0.5)
        .collect::<Vec<bool>>());

    println!("part 1: {}", gamma * epsilon);
}

fn num(v: &[bool]) -> usize {
    let mut value = 0;
    for &digit in v.iter() {
        value <<= 1;
        value += digit as usize;
    }
    value
}
