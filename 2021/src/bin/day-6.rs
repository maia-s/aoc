const INPUT: &str = include_str!("day-6.input");

fn main() {
    let mut fish = [0; 9];

    for s in INPUT.trim().split(',') {
        let n = s.parse::<usize>().unwrap();
        fish[n] += 1;
    }

    for _ in 0..80 {
        step(&mut fish);
    }
    println!("part 1: {}", fish.iter().sum::<usize>());
}

fn step(fish: &mut [usize; 9]) {
    let spawn = fish[0];
    for i in 0..8 {
        fish[i] = fish[i+1];
    }
    fish[6] += spawn;
    fish[8] = spawn;
}
