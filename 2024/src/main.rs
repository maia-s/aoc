use aoc_2024::Input;
use core::{
    fmt::{Debug, Display},
    hint::black_box,
    time::Duration,
};
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{LazyLock, Mutex},
    time::Instant,
};

const TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RUNS: usize = 50000;

static INPUTS: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    if let Ok(dir) = fs::read_dir(PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs"])) {
        for entry in dir.flatten() {
            let contents = fs::read_to_string(entry.path()).unwrap();
            let mut hash = String::new();
            for byte in Sha256::digest(&contents).into_iter() {
                hash.push_str(&format!("{byte:02x}"));
            }
            map.insert(hash, contents);
        }
    }
    Mutex::new(map)
});

fn get_input(input: Input) -> Option<Cow<'static, str>> {
    match input {
        Input::FileHash(hash) => INPUTS
            .lock()
            .unwrap()
            .get(hash)
            .map(|s| Cow::Owned(s.to_owned())),
        Input::Str(str) => Some(Cow::Borrowed(str)),
    }
}

macro_rules! days {
    ($($day:ident($maincfg:ident $(, $cfg:ident)* $(,)?)),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        $(
            fn $day() {
                println!("=== {} ===", stringify!($day));
                let input = get_input(aoc_2024::$day::$maincfg.input).expect("input not available");
                run("part 1", || aoc_2024::$day::part1(black_box(&input)));
                run("part 2", || aoc_2024::$day::part2(black_box(&input)));
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
                        let input = crate::get_input($maincfg.input).expect("input not available");
                        assert_eq!(part1(&input), $maincfg.part1_expected);
                        assert_eq!(part2(&input), $maincfg.part2_expected);
                    }

                    $(
                        #[test]
                        #[allow(non_snake_case)]
                        fn $cfg() {
                            use aoc_2024::$day::*;
                            let input = crate::get_input($cfg.input).expect("input not available");
                            assert_eq!(part1(&input), $cfg.part1_expected);
                            assert_eq!(part2(&input), $cfg.part2_expected);
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
    day3(INPUT, EX, EX2),
    day4(INPUT, EX),
}
