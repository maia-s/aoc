const INPUT: &str = include_str!("day-25.input");

fn main() {
    let mut it = INPUT.lines().map(|line| line.parse::<usize>().unwrap());
    let public_keys = [it.next().unwrap(), it.next().unwrap()];
    let loop_sizes = [loop_size(7, public_keys[0]), loop_size(7, public_keys[1])];

    let key = if loop_sizes[0] < loop_sizes[1] {
        transform(public_keys[1], loop_sizes[0])
    } else {
        transform(public_keys[0], loop_sizes[1])
    };

    println!("part 1: {}", key);
}

fn loop_size(subject_number: usize, target: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        value = (value * subject_number) % 20201227;
        if value == target {
            return loop_size;
        }
    }
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % 20201227;
    }
    value
}
