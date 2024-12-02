use core::{
    fmt::{Debug, Display},
    hint::black_box,
    time::Duration,
};
use std::time::Instant;

const TIMEOUT: Duration = Duration::from_secs(3);
const MAX_RUNS: usize = 50000;

macro_rules! days {
    ($($day:ident($maincfg:ident $(, $cfg:ident)* $(,)?)),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        $(
            fn $day() {
                println!("=== {} ===", stringify!($day));
                run("part 1", || aoc_2024::$day::part1(black_box(aoc_2024::$day::$maincfg.input)));
                run("part 2", || aoc_2024::$day::part2(black_box(aoc_2024::$day::$maincfg.input)));
            }
        )*

        fn main() {
            let args: Vec<String> = std::env::args().collect();
            match args.get(1).map(String::as_str) {
                $( Some(stringify!($day)) => $day(), )*
                Some(arg) => { eprintln!("unknown argument: `{arg}`") }
                None => { $(#[allow(unused)] let f = $day;)* f(); }
            }
        }

        #[cfg(test)]
        mod tests {
            $(
                mod $day {
                    #[test]
                    #[allow(non_snake_case)]
                    fn $maincfg() {
                        use aoc_2024::$day::*;
                        assert_eq!(part1($maincfg.input), $maincfg.part1_expected);
                        assert_eq!(part2($maincfg.input), $maincfg.part2_expected);
                    }

                    $(
                        #[test]
                        #[allow(non_snake_case)]
                        fn $cfg() {
                            use aoc_2024::$day::*;
                            assert_eq!(part1($cfg.input), $cfg.part1_expected);
                            assert_eq!(part2($cfg.input), $cfg.part2_expected);
                        }
                    )*
                }
            )*
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
    day1(INPUT, EX),
    day2(INPUT, EX, EDGE_CASE),
}
