const INPUT: &str = include_str!("day-9.input");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

#[derive(Default)]
struct Numbers {
    buffer: [u64; 25],
    i: usize,
}

impl Numbers {
    fn new(it: &mut impl Iterator<Item = u64>) -> Self {
        let mut numbers = Numbers::default();
        for i in numbers.buffer.iter_mut() {
            *i = it.next().unwrap();
        }
        numbers
    }

    fn add(&mut self, x: u64) -> bool {
        let mut valid = false;
        for i in 0..24 {
            for j in (i + 1)..25 {
                if self.buffer[i] + self.buffer[j] == x {
                    valid = true;
                    break;
                }
            }
            if valid {
                break;
            }
        }
        self.buffer[self.i] = x;
        self.i = (self.i + 1) % 25;
        valid
    }
}

fn part_1() -> u64 {
    let mut it = INPUT.lines().map(|s| s.parse().unwrap());
    let mut numbers = Numbers::new(&mut it);
    loop {
        let x = it.next().unwrap();
        if !numbers.add(x) {
            return x;
        }
    }
}

fn part_2() -> u64 {
    let target = part_1();
    let numbers: Vec<u64> = INPUT.lines().map(|s| s.parse().unwrap()).collect();
    let mut it = numbers.iter().copied();
    while let Some(x) = it.next() {
        let mut it2 = it.clone();
        let mut sum = x;
        let mut min = x;
        let mut max = x;
        while sum < target {
            let x = it2.next().unwrap();
            sum += x;
            min = min.min(x);
            max = max.max(x);
        }
        if sum == target {
            return min + max;
        }
    }
    panic!("not found");
}
