use core::{
    fmt::{Debug, Display},
    hint::black_box,
    time::Duration,
};
use std::time::Instant;

const TIMEOUT: Duration = Duration::from_secs(3);
const MAX_RUNS: usize = 10000;

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
    let mut times = Vec::with_capacity(MAX_RUNS);
    let result = f();
    println!("{name}: {result}");
    let t0 = Instant::now();
    let mut tc = t0;
    for _ in 0..MAX_RUNS {
        let tp = Instant::now();
        assert_eq!(black_box(f()), result);
        tc = Instant::now();
        times.push(tc.duration_since(tp).as_nanos() as u64);
        if tc.duration_since(t0) > TIMEOUT {
            break;
        }
    }
    times.sort_unstable();
    println!(
        "[ {}x avg:{:?} min:{:?} med:{:?} max:{:?} ]",
        times.len(),
        tc.duration_since(t0) / times.len() as u32,
        Duration::from_nanos(times[0]),
        Duration::from_nanos(times[times.len() / 2]),
        Duration::from_nanos(times[times.len() - 1]),
    );
}

days! {
    day1(2196996, 23655822),
}
