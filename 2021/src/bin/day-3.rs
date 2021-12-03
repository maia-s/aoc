const INPUT: &str = include_str!("day-3.input");

fn main() {
    let input: Vec<Vec<_>> = INPUT
        .lines()
        .map(|s| s.chars().map(|c| c as usize - b'0' as usize != 0).collect())
        .collect();
    let n = input.len();

    // part 1
    let sums = sums(&input);
    let gamma = num(&gammas(&sums, n));
    let epsilon = num(&epsilons(&sums, n));
    println!("part 1: {}", gamma * epsilon);

    // part 2
    let oxygen = part2(input.clone(), true);
    let scrubber = part2(input, false);
    println!("part 2: {}", oxygen * scrubber);
}

fn part2(mut input: Vec<Vec<bool>>, criteria: bool) -> usize {
    let mut i = 0;
    while input.len() > 1 {
        let n = input.len();
        let sums = sums(&input);
        let keep = if criteria {
            gammas(&sums, n)
        } else {
            epsilons(&sums, n)
        };
        input = input.into_iter().filter(|v| v[i] == keep[i]).collect();
        i += 1;
    }
    num(&input[0])
}

fn sums(input: &[Vec<bool>]) -> Vec<usize> {
    let digits = input[0].len();
    input.iter().fold(vec![0; digits], |mut sum, v| {
        for (s, &i) in sum.iter_mut().zip(v.iter()) {
            *s += i as usize;
        }
        sum
    })
}

fn gammas(sums: &[usize], n: usize) -> Vec<bool> {
    let n = n as f64;
    sums.iter().map(|&i| i as f64 / n >= 0.5).collect()
}

fn epsilons(sums: &[usize], n: usize) -> Vec<bool> {
    let n = n as f64;
    sums.iter().map(|&i| i as f64 / n < 0.5).collect()
}

fn num(v: &[bool]) -> usize {
    let mut value = 0;
    for &digit in v.iter() {
        value <<= 1;
        value += digit as usize;
    }
    value
}
