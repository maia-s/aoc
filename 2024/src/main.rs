macro_rules! days {
    ($($day:ident $(( $p1:expr $(, $p2:expr)? ))? ),* $(,)?) => {
        #[allow(non_upper_case_globals)]
        mod inputs {
            $( pub const $day: &str = include_str!(concat!(stringify!($day), ".txt")); )*
        }

        $(
            fn $day() {
                println!("=== {} ===", stringify!($day));
                println!("part 1: {}", aoc_2024::$day::part1(inputs::$day));
                println!("part 2: {}", aoc_2024::$day::part2(inputs::$day));
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

days! {
    day1(2196996, 23655822),
}
