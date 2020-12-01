const INPUT: &str = include_str!("day-1.input");

fn main() {
    let mut sums = [-1; 2020];

    for line in INPUT.lines() {
        let i: i32 = line.parse().expect("integer expected");
        let other = sums[i as usize];
        if other >= 0 {
            println!("{} + {} = {}", other, i, other + i);
            println!("{} * {} = {}", other, i, other * i);
            return;
        }
        sums[2020 - i as usize] = i;
    }

    eprintln!("not found");
}
