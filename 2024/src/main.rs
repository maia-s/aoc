use aoc_2024::Input;
use core::{
    fmt::{Debug, Display},
    hint::black_box,
    time::Duration,
};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    fs,
    io::{stdout, Write},
    path::PathBuf,
    sync::LazyLock,
    time::Instant,
};

const TIMEOUT: Duration = Duration::from_secs(10);
const MAX_RUNS: usize = 50000;

static INPUTS: LazyLock<HashMap<String, (String, String, String, String)>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    if let Ok(dir) = fs::read_dir(PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), "inputs"])) {
        for entry in dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with("part1") || name.ends_with("part2") {
                continue;
            }
            let path = entry.path();
            let contents = fs::read_to_string(&path).unwrap();
            let mut hash = String::new();
            for byte in Sha256::digest(&contents).into_iter() {
                hash.push_str(&format!("{byte:02x}"));
            }
            let mut p1 = path.clone();
            p1.set_extension("part1");
            let mut p2 = path.clone();
            p2.set_extension("part2");
            let part1 = fs::read_to_string(p1)
                .unwrap_or_default()
                .trim_end()
                .to_string();
            let part2 = fs::read_to_string(p2)
                .unwrap_or_default()
                .trim_end()
                .to_string();
            map.insert(hash, (name, contents, part1, part2));
        }
    }
    map
});

fn get_input<T: ToString, U: ToString>(
    input: &Input<T, U>,
) -> Option<(String, String, Option<String>, Option<String>)> {
    match input {
        Input::Hashed(hash) => INPUTS.get(*hash).map(|(n, s, p1, p2)| {
            (
                n.clone(),
                s.clone(),
                (!p1.is_empty()).then(|| p1.to_string()),
                (!p2.is_empty()).then(|| p2.to_string()),
            )
        }),
        Input::Inline(name, str, p1, p2) => Some((
            name.to_string(),
            str.to_string(),
            p1.as_ref().map(|p1| p1.to_string()),
            p2.as_ref().map(|p2| p2.to_string()),
        )),
    }
}

macro_rules! days {
    ($($day:ident),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        $(
            fn $day() {
                let (name, input, _, _) = get_input(&aoc_2024::$day::inputs()[0]).expect("input not available");
                println!("=== {}: {name} ===", stringify!($day));
                println!("part 1: {}", aoc_2024::$day::part1(&input));
                println!("part 2: {}", aoc_2024::$day::part2(&input));
                println!(
                    "                {:>10} {:>10} {:>10} {:>10}",
                    "- avg -",
                    "- min -",
                    "- med -",
                    "- max -"
                );
                run("part 1", || aoc_2024::$day::part1(black_box(&input)));
                run("part 2", || aoc_2024::$day::part2(black_box(&input)));
            }
        )*

        fn main() {
            let args: Vec<String> = std::env::args().collect();
            for arg in 1.. {
                match args.get(arg).map(String::as_str) {
                    $( Some(stringify!($day)) => $day(), )*
                    Some("all") => { $($day();)* }
                    Some(arg) => { eprintln!("unknown argument: `{arg}`") }
                    None => { if arg == 1 { $(#[allow(unused)] let f = $day;)* f(); } break }
                }
            }
        }

        #[cfg(test)]
        mod tests {
            $(
                mod $day {
                    #[test]
                    fn part1() {
                        for input in aoc_2024::$day::inputs() {
                            let (name, input, p1, _) = crate::get_input(&input).expect("input not available");
                            if let Some(p1) = p1 {
                                assert_eq!(aoc_2024::$day::part1(&input).to_string(), p1, "{name}");
                            } else {
                                eprintln!("n/a: {name}");
                            }
                        }
                    }

                    #[test]
                    fn part2() {
                        for input in aoc_2024::$day::inputs() {
                            let (name, input, _, p2) = crate::get_input(&input).expect("input not available");
                            if let Some(p2) = p2 {
                                assert_eq!(aoc_2024::$day::part2(&input).to_string(), p2, "{name}");
                            } else {
                                eprintln!("n/a: {name}");
                            }
                        }
                    }
                }
            )*
        }
    };
}

fn run<R: Debug + Display + PartialEq>(name: &str, f: impl Fn() -> R) {
    let mut times = Vec::with_capacity(MAX_RUNS);
    let result = f();
    let t0 = Instant::now();
    let mut tc = t0;
    print!("{name}");
    let _ = stdout().flush();
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
        " [ {:>5}x {:>10.3?} {:>10.3?} {:>10.3?} {:>10.3?} ]",
        times.len(),
        tc.duration_since(t0) / times.len() as u32,
        Duration::from_nanos(times[0]),
        Duration::from_nanos(times[times.len() / 2]),
        Duration::from_nanos(times[times.len() - 1]),
    );
}

days! {
    day1,
    day2,
    day3,
    day4,
    day5,
    day6,
    day6_simd,
    day7,
    day8,
    day9,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
}
