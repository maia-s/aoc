const INPUT: &str = include_str!("day-1.input");

fn main() {
    let input: Vec<_> = INPUT.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    // part 1
    println!(
        "part 1: {}",
        input
            .windows(2)
            .fold(0, |n, i| if i[1] > i[0] { n + 1 } else { n })
    );

    // part 2
    println!(
        "part 2: {}",
        input
            .windows(4)
            .fold(0, |n, i| if i[3] > i[0] { n + 1 } else { n })
    );
}
