const INPUT: &str = include_str!("day-1.input");

fn main() {
    let mut nums = Vec::<i32>::new();

    for line in INPUT.lines() {
        nums.push(line.parse().expect("integer expected"));
    }

    let mut sums = [None; 2020];

    for &i in nums.iter() {
        for &j in nums.iter() {
            let ij = i + j;
            if ij < 2020 {
                sums[2020 - ij as usize] = Some((i, j));
            }
        }
    }

    for &i in nums.iter() {
        if let Some((a, b)) = sums[i as usize] {
            println!("{} + {} + {} = {}", a, b, i, a + b + i);
            println!("{} * {} * {} = {}", a, b, i, a * b * i);
            return;
        }
    }

    eprintln!("not found");
}
