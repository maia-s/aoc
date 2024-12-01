pub mod day1;

pub struct Conf<T, U = T> {
    pub input: &'static str,
    pub part1_expected: T,
    pub part2_expected: U,
}

impl<T, U> Conf<T, U> {
    pub const fn new(input: &'static str, part1_expected: T, part2_expected: U) -> Self {
        Self {
            input,
            part1_expected,
            part2_expected,
        }
    }
}
