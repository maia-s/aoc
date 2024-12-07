#![feature(portable_simd)]

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

pub enum Input {
    FileHash(&'static str),
    Str(&'static str),
}

pub struct Conf<T = u32, U = T> {
    pub input: Input,
    pub part1_expected: T,
    pub part2_expected: U,
}

impl<T, U> Conf<T, U> {
    pub const fn new(input: Input, part1_expected: T, part2_expected: U) -> Self {
        Self {
            input,
            part1_expected,
            part2_expected,
        }
    }
}
