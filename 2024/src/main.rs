use core::{
    fmt::{Debug, Display},
    hint::black_box,
    time::Duration,
};
use std::time::Instant;

macro_rules! days {
    ($($day:ident $(( $p1:expr $(, $p2:expr)? ))? ),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        mod inputs {
            $( pub const $day: &str = include_str!(concat!(stringify!($day), ".txt")); )*
        }

        $(
            fn $day() {
                println!("=== {} ===", stringify!($day));
                run("part 1", || aoc_2024::$day::part1(black_box(inputs::$day)));
                run("part 2", || aoc_2024::$day::part2(black_box(inputs::$day)));
            }
        )*

        fn main() {
            let args: Vec<String> = std::env::args().collect();
            match args.get(1).map(String::as_str) {
                $( Some(stringify!($day)) => $day(), )*
                Some(arg) => { eprintln!("unknown argument: `{arg}`") }
                None => { $(let f = $day;)* f(); }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            $($(
                #[test]
                fn $day() {
                    assert_eq!(aoc_2024::$day::part1(inputs::$day), $p1);
                    $( assert_eq!(aoc_2024::$day::part2(inputs::$day), $p2); )?
                }
            )?)*
        }
    };
}

fn run<R: Debug + Display + PartialEq>(name: &str, f: impl Fn() -> R) {
    const TIMEOUT: Duration = Duration::from_secs(3);
    let mut times: [u64; 1000] = [0; 1000];
    let result = f();
    println!("{name}: {result}");
    let mut n = 0_u32;
    let t0 = Instant::now();
    let mut tc = t0;
    for time in times.iter_mut() {
        let tp = Instant::now();
        assert_eq!(black_box(f()), result);
        tc = Instant::now();
        *time = tc.duration_since(tp).as_nanos() as u64;
        n += 1;
        if tc.duration_since(t0) > TIMEOUT {
            break;
        }
    }
    times[..n as usize].sort_unstable();
    println!(
        "[ {n}x avg:{:#?} min:{:#?} med:{:#?} max:{:#?} ]",
        tc.duration_since(t0) / n,
        Duration::from_nanos(times[0]),
        Duration::from_nanos(times[n as usize / 2]),
        Duration::from_nanos(times[n as usize - 1]),
    );
}

days! {
    day1(2196996, 23655822),
}
