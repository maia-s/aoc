const INPUT: &str = include_str!("day-1.input");

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    println!("=[ part 1 ]=");

    let mut sums = [-1; 2020];

    for line in INPUT.lines() {
        let i: i32 = line.parse().expect("expected i32");
        let other = sums[i as usize];
        if other >= 0 {
            println!("{} + {} = {}", other, i, other + i);
            println!("{} * {} = {}", other, i, other * i);
            return;
        }
        sums[2020 - i as usize] = i;
    }

    println!("not found");
}

fn part_2() {
    println!("\n=[ part 2 ]=");

    let mut nums = Vec::<i32>::new();

    for line in INPUT.lines() {
        nums.push(line.parse().expect("expected i32"));
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

    println!("not found");
}
