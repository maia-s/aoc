use std::str::FromStr;

const INPUT: &str = include_str!("day-8.input");

#[derive(Clone, Copy)]
struct Digit(u8);

impl Digit {
    fn number_of_segments(&self) -> usize {
        self.0.count_ones() as usize
    }
}

impl FromStr for Digit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = 0;
        for c in s.chars() {
            let bit = (c as usize - 'a' as usize) as usize;
            assert!(bit < 7);
            segments |= 1 << bit;
        }
        Ok(Self(segments))
    }
}

struct Digits(Vec<Digit>);

impl FromStr for Digits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.trim().split(' ').map(|s| s.parse().unwrap()).collect(),
        ))
    }
}

fn main() {
    let input: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let (signals, segments) = line.split_once('|').unwrap();
            (
                signals.parse::<Digits>().unwrap().0,
                segments.parse::<Digits>().unwrap().0,
            )
        })
        .collect();

    // part 1
    println!(
        "part 1: {}",
        input
            .iter()
            .map(|(_, s)| s
                .iter()
                .map(Digit::number_of_segments)
                .filter(|n| matches!(n, 2 | 3 | 4 | 7))
                .count())
            .sum::<usize>()
    );

    // part 2
    let mut sum = 0;
    for (signals, digits) in input {
        let mut on = [0_u8; 10];
        let mut m5 = 0x7f;
        let mut m6 = 0x7f;

        for &signal in signals.iter() {
            match signal.number_of_segments() {
                2 => {
                    // 1
                    on[1] = signal.0;
                }
                3 => {
                    // 7
                    on[7] = signal.0;
                }
                4 => {
                    // 4
                    on[4] = signal.0;
                }
                5 => {
                    // 2, 3 or 5
                    m5 &= signal.0;
                }
                6 => {
                    // 0, 6 or 9
                    m6 &= signal.0;
                }
                7 => {
                    // 8
                    on[8] = 0x7f;
                }
                _ => unreachable!(),
            }
        }

        on[3] = m5;
        on[0] = m6;
        //let a = on[1] ^ on[7];
        let d = on[3] & on[4];
        on[0] = on[8] ^ d;
        on[3] |= on[1];
        // 0 1 3 4 7 8
    
        for &signal in signals.iter() {
            match signal.number_of_segments() {
                2 | 3 | 4 | 7 => (),
                5 => {
                    // 2, 3 or 5
                    if signal.0 != on[3] {
                        // 2 or 5
                        if (signal.0 & on[4]).count_ones() == 3 {
                            on[5] = signal.0;
                        } else {
                            on[2] = signal.0;
                        }
                    }
                }
                6 => {
                    // 0, 6 or 9
                    if signal.0 != on[0] {
                        // 6 or 9
                        if (signal.0 & on[1]).count_ones() == 2 {
                            on[9] = signal.0;
                        } else {
                            on[6] = signal.0;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        let mut val = 0;
        for &digit in digits.iter() {
            for (i, &o) in on.iter().enumerate() {
                if o == digit.0 {
                    val = val * 10 + i;
                    break;
                }
            }
        }
        sum += val;
    }

    println!("part 2: {}", sum);
}
